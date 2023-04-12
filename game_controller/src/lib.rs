//! This crate contains the core functionality of the GameController. This comprises:
//! - definition of states and timers
//! - applying actions to a state in response to external events or (user-controlled) passing of time
//! - logging
//!
//! It does not contain a user interface and does not handle network packets.

pub mod action;
pub mod actions;
pub mod log;
pub mod timer;
pub mod types;

use std::{cmp::min, iter::once, time::Duration};

use enum_map::enum_map;

use crate::action::VAction;
use crate::log::{LogEntry, LoggedAction, Logger, TimestampedLogEntry};
use crate::timer::{BehaviorAtZero, EvaluatedRunConditions, RunCondition, Timer};
use crate::types::{
    ActionSource, Game, Params, Penalty, Phase, Player, PlayerNumber, SetPlay, Side, SideMapping,
    State, Team,
};

/// This struct handles the main logic of the GameController.
pub struct GameController {
    /// The constant parameters of the game.
    pub params: Params,
    game: Game,
    delayed_game: Option<Game>,
    delayed_game_timer: Timer,
    time: Duration,
    logger: Box<dyn Logger + Send>,
}

impl GameController {
    /// This function creates a new instance with given parameters and a logger.
    pub fn new(params: Params, logger: Box<dyn Logger + Send>) -> Self {
        let game = Game {
            sides: SideMapping::HomeDefendsLeftGoal,
            phase: Phase::FirstHalf,
            state: State::Initial,
            set_play: SetPlay::NoSetPlay,
            kicking_side: Side::Home,
            primary_timer: Timer::Started {
                remaining: params.competition.half_duration.try_into().unwrap(),
                run_condition: RunCondition::Playing,
                behavior_at_zero: BehaviorAtZero::Overflow,
            },
            secondary_timer: Timer::Stopped,
            teams: enum_map! {
                _ => Team {
                    goalkeeper: PlayerNumber::new(1),
                    score: 0,
                    penalty_counter: 0,
                    timeout_budget: params.competition.timeouts_per_team,
                    message_budget: params.competition.messages_per_team,
                    illegal_communication: false,
                    penalty_shot: 0,
                    penalty_shot_mask: 0,
                    players: (PlayerNumber::MIN..=PlayerNumber::MAX)
                        .map(|player| Player {
                            // By default, the higher-numbered players are substitutes.
                            penalty: if player <= params.competition.players_per_team {
                                Penalty::NoPenalty
                            } else {
                                Penalty::Substitute
                            },
                            penalty_timer: Timer::Stopped,
                        })
                        // We have to collect into a Vec first because this thing cannot be directly
                        // collected into a fixed size array.
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                },
            },
        };
        Self {
            params,
            game,
            delayed_game: None,
            delayed_game_timer: Timer::Stopped,
            time: Duration::ZERO,
            logger,
        }
    }

    /// This function returns the dynamic state of the game. The caller can request if the game
    /// state should be the delayed game state.
    pub fn get_game(&self, delayed: bool) -> &Game {
        if delayed {
            self.delayed_game.as_ref().unwrap_or(&self.game)
        } else {
            &self.game
        }
    }

    /// This function lets time progress. Timers are updated and expiration actions applied when
    /// necessary.
    pub fn seek(&mut self, mut dt: Duration) {
        // We must split the time when timers expire in the meantime, because they can have actions
        // which must be applied at the right point in time.
        while !dt.is_zero() {
            // Find out how far we can seek at most in this iteration.
            let this_dt = self.clip_next_timer_expiration(dt);

            // Update current time and remaining offset.
            self.time += this_dt;
            dt -= this_dt;

            let run_conditions = EvaluatedRunConditions::new(&self.game, &self.params);

            // Update delayed game state. This is not really elegant at the moment.
            self.delayed_game_timer.seek(this_dt, &run_conditions);
            if matches!(self.delayed_game_timer, Timer::Stopped) {
                self.delayed_game = None;
            } else if let Some(delayed_game) = self.delayed_game.as_mut() {
                let delayed_run_conditions =
                    EvaluatedRunConditions::new(delayed_game, &self.params);
                let delayed_actions: Vec<VAction> = delayed_game
                    .timers_mut()
                    .filter_map(|timer| timer.seek(this_dt, &delayed_run_conditions))
                    .flatten()
                    .collect();
                for action in delayed_actions {
                    self.apply_delayed(&action);
                }
            }

            // Seek timers and obtain actions of timers that expire at the end of this iteration.
            let actions: Vec<VAction> = self
                .game
                .timers_mut()
                .filter_map(|timer| timer.seek(this_dt, &run_conditions))
                .flatten()
                .collect();

            for action in actions {
                self.apply(action, ActionSource::Timer);
            }
        }
    }

    /// This function applies an action, given that it is legal. Some special cases will be
    /// filtered here. The action as well as the resulting game state is logged.
    pub fn apply(&mut self, action: VAction, source: ActionSource) {
        if !action.is_legal(&self.game, &self.params) {
            return;
        }

        // Some actions are intercepted here because their effects are delayed. Only actions that
        // are not triggered by a timer are forwarded to the delayed state, because timers may
        // differ and timers from the delayed state call directly into the corresponding function.
        // I'm not sure if timer-triggered actions from the non-delayed state should still cancel
        // the delayed state if they are illegal.
        if self.game.phase != Phase::PenaltyShootout && matches!(action, VAction::Goal(_)) {
            self.hide(self.params.competition.delay_after_goal);
        } else if matches!(
            action,
            VAction::FreePenaltyShot(_) | VAction::FreeSetPlay(_)
        ) {
            self.hide(self.params.competition.delay_after_playing);
        } else if source != ActionSource::Timer {
            self.apply_delayed(&action);
        }

        action.execute(&mut self.game, &self.params);
        self.log_now(LogEntry::Action(LoggedAction { source, action }));
        self.log_now(LogEntry::GameState(Box::new(self.game.clone())));
    }

    /// This function adds an entry to the log file at the current point in time.
    pub fn log_now(&mut self, entry: LogEntry) {
        self.logger.append(TimestampedLogEntry {
            timestamp: self.time,
            entry,
        });
    }

    /// This function clips a given timestamp (given as duration from now) to the time when the
    /// next timer expires.
    pub fn clip_next_timer_expiration(&self, max: Duration) -> Duration {
        let run_conditions = EvaluatedRunConditions::new(&self.game, &self.params);
        self.game
            .timers()
            .chain(once(&self.delayed_game_timer))
            .filter(|timer| timer.will_expire() && timer.is_running(&run_conditions))
            // At this point, we can be sure that the remaining time is not negative.
            .map(|timer| timer.get_remaining().try_into().unwrap())
            .chain(
                self.delayed_game
                    .as_ref()
                    .map(|delayed_game| {
                        let delayed_run_conditions =
                            EvaluatedRunConditions::new(delayed_game, &self.params);
                        delayed_game
                            .timers()
                            .filter(move |timer| {
                                timer.will_expire() && timer.is_running(&delayed_run_conditions)
                            })
                            // At this point, we can be sure that the remaining time is not
                            // negative.
                            .map(|timer| timer.get_remaining().try_into().unwrap())
                    })
                    .into_iter()
                    .flatten(),
            )
            .min()
            .map_or(max, |next| min(next, max))
    }

    /// This function clips a given timestamp (given as duration from now) to the time when the
    /// next timer, when rounded to seconds in a particular way, changes.
    pub fn clip_next_timer_wrap(&self, max: Duration) -> Duration {
        let next_wrap = |timer: &Timer| {
            Duration::from_nanos(
                ({
                    let nanos = timer.get_remaining().subsec_nanoseconds();
                    if nanos <= 0 {
                        nanos + 1_000_000_000
                    } else {
                        nanos
                    }
                })
                .try_into()
                .unwrap(),
            )
        };
        let run_conditions = EvaluatedRunConditions::new(&self.game, &self.params);
        self.game
            .timers()
            .filter(|timer| timer.is_running(&run_conditions))
            .map(next_wrap)
            .chain(
                self.delayed_game
                    .as_ref()
                    .map(|delayed_game| {
                        let delayed_run_conditions =
                            EvaluatedRunConditions::new(delayed_game, &self.params);
                        delayed_game
                            .timers()
                            .filter(move |timer| timer.is_running(&delayed_run_conditions))
                            .map(next_wrap)
                    })
                    .into_iter()
                    .flatten(),
            )
            .min()
            .map_or(max, |next| min(next, max))
    }

    fn apply_delayed(&mut self, action: &VAction) {
        if let Some(delayed_game) = self.delayed_game.as_mut() {
            // FinishSetPlay is not a reason to cancel the delayed state because that would mean
            // that e.g. a kick-off is delayed for only 10 seconds.
            if action.is_legal(delayed_game, &self.params) {
                action.execute(delayed_game, &self.params);
            } else if !matches!(action, VAction::FinishSetPlay(_)) {
                self.delayed_game = None;
                self.delayed_game_timer = Timer::Stopped;
            }
        }
    }

    fn hide(&mut self, duration: Duration) {
        self.delayed_game_timer = Timer::Started {
            remaining: duration.try_into().unwrap(),
            run_condition: RunCondition::Always,
            behavior_at_zero: BehaviorAtZero::Expire(vec![]),
        };
        self.delayed_game = Some(self.game.clone());
    }
}

impl Drop for GameController {
    fn drop(&mut self) {
        self.log_now(LogEntry::End);
    }
}
