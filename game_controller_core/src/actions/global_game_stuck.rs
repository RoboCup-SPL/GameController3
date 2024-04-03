use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::actions::StartSetPlay;
use crate::types::{Phase, SetPlay, State};

/// This struct defines an action which corresponds to the referee call "Global Game Stuck".
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GlobalGameStuck;

impl Action for GlobalGameStuck {
    fn execute(&self, c: &mut ActionContext) {
        StartSetPlay {
            side: c.game.next_global_game_stuck_kick_off,
            set_play: SetPlay::KickOff,
        }
        .execute(c);
        c.game.next_global_game_stuck_kick_off = -c.game.next_global_game_stuck_kick_off;
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.phase != Phase::PenaltyShootout && c.game.state == State::Playing
    }
}
