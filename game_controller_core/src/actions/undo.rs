use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};

/// This struct defines an action which reverts the game to a previous state.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Undo {
    /// The number of previous user actions to be reverted.
    pub states: u32,
}

impl Action for Undo {
    fn execute(&self, _c: &mut ActionContext) {}

    fn is_legal(&self, _c: &ActionContext) -> bool {
        true
    }
}
