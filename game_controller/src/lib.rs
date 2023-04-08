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

use std::{cmp::min, time::Duration};

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
    /// The current dynamic state of the game.
    pub game: Game,
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
            time: Duration::ZERO,
            logger,
        }
    }

    /// This function lets time progress. Timers are updated and expiration actions applied when
    /// necessary.
    pub fn seek(&mut self, mut dt: Duration) {
        // We must split the time when timers expire in the meantime, because they can have actions
        // which must be applied at the right point in time.
        while !dt.is_zero() {
            let run_conditions = EvaluatedRunConditions::new(&self.game, &self.params);

            // Find out how far we can seek at most in this iteration.
            let this_dt = self.clip_next_timer_expiration(&run_conditions, dt);

            // Update current time and remaining offset.
            self.time += this_dt;
            dt -= this_dt;

            // Seek timers and obtain actions of timers that expire at the end of this iteration.
            let actions: Vec<VAction> = self
                .timers_mut()
                .iter_mut()
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
        if !action.is_legal(&self.game) {
            return;
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
    pub fn clip_next_timer_expiration(
        &self,
        run_conditions: &EvaluatedRunConditions,
        max: Duration,
    ) -> Duration {
        if let Some(next) = self
            .timers()
            .iter()
            .filter(|timer| timer.will_expire() && timer.is_running(run_conditions))
            // At this point, we can be sure that the remaining time is not negative.
            .map(|timer| timer.get_remaining().try_into().unwrap())
            .min()
        {
            min(next, max)
        } else {
            max
        }
    }

    /// This function clips a given timestamp (given as duration from now) to the time when the
    /// next timer, when rounded to seconds in a particular way, changes.
    pub fn clip_next_timer_wrap(
        &self,
        run_conditions: &EvaluatedRunConditions,
        max: Duration,
    ) -> Duration {
        if let Some(next) = self
            .timers()
            .iter()
            .filter(|timer| timer.is_running(run_conditions))
            .map(|timer| {
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
            })
            .min()
        {
            min(next, max)
        } else {
            max
        }
    }

    fn timers(&self) -> Vec<&Timer> {
        let mut timers: Vec<&Timer> = vec![&self.game.primary_timer, &self.game.secondary_timer];
        let mut penalty_timers: Vec<&Timer> = self
            .game
            .teams
            .values()
            .flat_map(|team| team.players.iter().map(|player| &player.penalty_timer))
            .collect();
        timers.append(&mut penalty_timers);
        timers
    }

    fn timers_mut(&mut self) -> Vec<&mut Timer> {
        let mut timers = vec![&mut self.game.primary_timer, &mut self.game.secondary_timer];
        let mut penalty_timers: Vec<&mut Timer> = self
            .game
            .teams
            .values_mut()
            .flat_map(|team| {
                team.players
                    .iter_mut()
                    .map(|player| &mut player.penalty_timer)
            })
            .collect();
        timers.append(&mut penalty_timers);
        timers
    }
}

impl Drop for GameController {
    fn drop(&mut self) {
        self.log_now(LogEntry::End);
    }
}
