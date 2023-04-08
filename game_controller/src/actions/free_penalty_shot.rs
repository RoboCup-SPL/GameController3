use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::types::{Game, Params, Phase, State};

/// This struct defines an action which corresponds to the referee call "Playing" in a penalty
/// shoot-out.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FreePenaltyShot;

impl Action for FreePenaltyShot {
    fn execute(&self, game: &mut Game, _params: &Params) {
        game.state = State::Playing;
    }

    fn is_legal(&self, game: &Game) -> bool {
        game.phase == Phase::PenaltyShootout && game.state == State::Set
    }
}
