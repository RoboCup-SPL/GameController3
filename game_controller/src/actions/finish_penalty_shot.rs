use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::types::{Phase, State};

/// This struct defines an action which corresponds to the referee call "Finish" in a penalty
/// shoot-out.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FinishPenaltyShot;

impl Action for FinishPenaltyShot {
    fn execute(&self, c: &mut ActionContext) {
        c.game.state = State::Finished;
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.phase == Phase::PenaltyShootout && c.game.state == State::Playing
    }
}
