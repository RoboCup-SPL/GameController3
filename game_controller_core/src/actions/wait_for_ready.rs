use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::types::{Phase, State};

/// This struct defines an action which corresponds to the referee call "Standby".
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WaitForReady;

impl Action for WaitForReady {
    fn execute(&self, c: &mut ActionContext) {
        c.game.state = State::Standby;
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        !c.params.competition.delay_after_ready.is_zero()
            && c.game.phase != Phase::PenaltyShootout
            && (c.game.state == State::Initial || c.game.state == State::Timeout)
    }
}
