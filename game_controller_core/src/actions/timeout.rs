use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{Phase, SetPlay, Side, State};

/// This struct defines an action for when a team or the referee takes a timeout.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Timeout {
    /// The side which takes the timeout or [None] for a referee timeout.
    pub side: Option<Side>,
}

impl Action for Timeout {
    fn execute(&self, c: &mut ActionContext) {
        // Cancel all penalty timers.
        c.game.teams.values_mut().for_each(|team| {
            team.players.iter_mut().for_each(|player| {
                player.penalty_timer = Timer::Stopped;
            })
        });

        if c.game.phase != Phase::PenaltyShootout {
            // If this is not a referee timeout, the next kick-off is for the other team.
            if let Some(side) = self.side {
                c.game.kicking_side = -side;
            }
            // The primary timer is rewound to the time when the stoppage of play has started.
            c.game.primary_timer = Timer::Started {
                remaining: c.game.primary_timer.get_remaining()
                    - c.game.timeout_rewind_timer.get_remaining(),
                run_condition: RunCondition::Playing,
                behavior_at_zero: BehaviorAtZero::Overflow,
            };
            c.game.timeout_rewind_timer = Timer::Stopped;
        }
        let duration = if self.side.is_some() {
            c.params.competition.timeout_duration
        } else {
            c.params.competition.referee_timeout_duration
        };
        c.game.secondary_timer = Timer::Started {
            // In some cases, an existing timer is modified to avoid situations like "We are going
            // to take a timeout once their timeout is over".
            remaining: if c.game.state == State::Timeout
                || (c.game.state == State::Initial && c.game.phase == Phase::SecondHalf)
            {
                c.game.secondary_timer.get_remaining() + duration
            } else {
                duration.try_into().unwrap()
            },
            run_condition: RunCondition::Always,
            behavior_at_zero: BehaviorAtZero::Overflow,
        };
        c.game.state = State::Timeout;
        c.game.set_play = SetPlay::NoSetPlay;
        if let Some(side) = self.side {
            c.game.teams[side].timeout_budget -= 1;
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.state != State::Playing
            && c.game.state != State::Finished
            && (c.game.phase != Phase::PenaltyShootout
                || c.game.state == State::Initial
                || c.game.state == State::Timeout)
            // This check is so you can't take timeouts during a penalty kick Ready/Set. The rules
            // don't explicitly rule this out (I think), but it would be ridiculous if it was
            // legal.
            && (c.game.set_play == SetPlay::NoSetPlay || c.game.set_play == SetPlay::KickOff)
            && self.side.map_or(true, |side| c.game.teams[side].timeout_budget > 0)
    }
}
