use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{Game, Params, Phase, State};

/// This struct defines an action which corresponds to the referee call "Set" in a penalty
/// shoot-out.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WaitForPenaltyShot;

impl Action for WaitForPenaltyShot {
    fn execute(&self, game: &mut Game, params: &Params) {
        if game.state != State::Initial {
            game.sides = -game.sides;
            game.kicking_side = -game.kicking_side;
        }
        game.state = State::Set;
        game.primary_timer = Timer::Started {
            remaining: params.competition.penalty_shot_duration.try_into().unwrap(),
            run_condition: RunCondition::Playing,
            behavior_at_zero: BehaviorAtZero::Overflow,
        };
        game.teams[game.kicking_side].penalty_shot += 1;
    }

    fn is_legal(&self, game: &Game) -> bool {
        game.phase == Phase::PenaltyShootout
            && (game.state == State::Initial || game.state == State::Finished)
    }
}
