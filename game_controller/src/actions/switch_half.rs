use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{Game, Params, Phase, State};

/// This struct defines an action that switches from the end of the first half to the beginning of
/// the second half, including the switch of sides.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SwitchHalf;

impl Action for SwitchHalf {
    fn execute(&self, game: &mut Game, params: &Params) {
        game.sides = -params.game.side_mapping;
        game.phase = Phase::SecondHalf;
        game.state = State::Initial;
        game.kicking_side = -params.game.kick_off_side;

        game.primary_timer = Timer::Started {
            remaining: params.competition.half_duration.try_into().unwrap(),
            run_condition: RunCondition::Playing,
            behavior_at_zero: BehaviorAtZero::Overflow,
        };
    }

    fn is_legal(&self, game: &Game, _params: &Params) -> bool {
        game.phase == Phase::FirstHalf && game.state == State::Finished
    }
}
