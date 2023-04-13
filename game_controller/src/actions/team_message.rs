use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::types::{Game, Params, Phase, Side, State};

/// This struct defines an action that is triggered when a team message is received.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamMessage {
    /// The side which sent the team message.
    pub side: Side,
    /// Whether the message itself has an illegal format.
    pub illegal: bool,
}

impl Action for TeamMessage {
    fn execute(&self, game: &mut Game, _params: &Params) {
        if game.teams[self.side].message_budget == 0 || self.illegal {
            game.teams[self.side].illegal_communication = true;
            game.teams[self.side].score = 0;
        } else {
            game.teams[self.side].message_budget -= 1;
        }
    }

    fn is_legal(&self, game: &Game, _params: &Params) -> bool {
        // Team messages are only counted during those states.
        game.phase != Phase::PenaltyShootout
            && (game.state == State::Ready
                || game.state == State::Set
                || game.state == State::Playing)
    }
}
