use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::types::{Game, Params, Phase, State};

/// This struct defines an action which corresponds to the referee call "Finish" in a penalty
/// shoot-out.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FinishPenaltyShot;

impl Action for FinishPenaltyShot {
    fn execute(&self, game: &mut Game, _params: &Params) {
        game.state = State::Finished;
    }

    fn is_legal(&self, game: &Game, _params: &Params) -> bool {
        game.phase == Phase::PenaltyShootout && game.state == State::Playing
    }
}
