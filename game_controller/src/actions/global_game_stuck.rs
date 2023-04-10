use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::actions::StartSetPlay;
use crate::types::{Game, Params, Phase, SetPlay, Side, State};

/// This struct defines an action which corresponds to the referee call "Global Game Stuck".
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalGameStuck {
    /// The side which caused the global game stuck. A kick-off is awarded to the other team.
    pub side: Side,
}

impl Action for GlobalGameStuck {
    fn execute(&self, game: &mut Game, params: &Params) {
        StartSetPlay {
            side: -self.side,
            set_play: SetPlay::KickOff,
        }
        .execute(game, params);
    }

    fn is_legal(&self, game: &Game, _params: &Params) -> bool {
        game.phase != Phase::PenaltyShootout && game.state == State::Playing
    }
}
