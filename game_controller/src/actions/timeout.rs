use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{Game, Params, Phase, SetPlay, Side, State};

/// This struct defines an action for when a team takes a timeout.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Timeout {
    /// The side which takes the timeout.
    pub side: Side,
}

impl Action for Timeout {
    fn execute(&self, game: &mut Game, params: &Params) {
        // Cancel all penalty timers.
        game.teams.values_mut().for_each(|team| {
            team.players.iter_mut().for_each(|player| {
                player.penalty_timer = Timer::Stopped;
            })
        });

        if game.phase != Phase::PenaltyShootout {
            // The next kick-off is for the other team.
            game.kicking_side = -self.side;
        }
        game.secondary_timer = Timer::Started {
            // In some cases, an existing timer is modified to avoid situations like "We are going
            // to take a timeout once their timeout is over".
            remaining: if game.state == State::Timeout
                || (game.state == State::Initial && game.phase == Phase::SecondHalf)
            {
                game.secondary_timer.get_remaining() + params.competition.timeout_duration
            } else {
                params.competition.timeout_duration.try_into().unwrap()
            },
            run_condition: RunCondition::Always,
            behavior_at_zero: BehaviorAtZero::Overflow,
        };
        game.state = State::Timeout;
        game.set_play = SetPlay::NoSetPlay;
        game.teams[self.side].timeout_budget -= 1;
    }

    fn is_legal(&self, game: &Game, _params: &Params) -> bool {
        game.state != State::Playing
            && game.state != State::Finished
            && (game.phase != Phase::PenaltyShootout
                || game.state == State::Initial
                || game.state == State::Timeout)
            // This check is so you can't take timeouts during a penalty kick Ready/Set. The rules
            // don't explicitly rule this out (I think), but it would be ridiculous if it was
            // legal.
            && (game.set_play == SetPlay::NoSetPlay || game.set_play == SetPlay::KickOff)
            && game.teams[self.side].timeout_budget > 0
    }
}
