use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::types::State;

/// This struct defines an action which corresponds to the referee call "Initial".
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StartHalf;

impl Action for StartHalf {
    fn execute(&self, c: &mut ActionContext) {
        c.game.state = State::Initial;
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.state == State::Setup
    }
}
