use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{Game, Params, Phase, SetPlay, State};

/// This struct defines an action which corresponds to the referee call "Finish" (or rather
/// two/three successive whistles).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FinishHalf;

impl Action for FinishHalf {
    fn execute(&self, game: &mut Game, params: &Params) {
        // Cancel all penalty timers.
        game.teams.values_mut().for_each(|team| {
            team.players.iter_mut().for_each(|player| {
                player.penalty_timer = Timer::Stopped;
            })
        });

        game.secondary_timer = Timer::Stopped;
        game.set_play = SetPlay::NoSetPlay;
        game.state = State::Finished;

        // After the first half, a timer counts down the half-time break.
        if game.phase == Phase::FirstHalf {
            game.secondary_timer = Timer::Started {
                remaining: params
                    .competition
                    .half_time_break_duration
                    .try_into()
                    .unwrap(),
                run_condition: RunCondition::Always,
                behavior_at_zero: BehaviorAtZero::Overflow,
            };
        }
    }

    fn is_legal(&self, game: &Game) -> bool {
        game.phase != Phase::PenaltyShootout
            && (game.state == State::Playing
                || game.state == State::Ready
                || game.state == State::Set)
    }
}
