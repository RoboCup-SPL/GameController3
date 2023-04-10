use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::timer::Timer;
use crate::types::{Game, Params, Penalty, PlayerNumber, Side, State};

/// This struct defines an action to unpenalize players.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Unpenalize {
    /// The side whose player is unpenalized.
    pub side: Side,
    /// The number of the player who is unpenalized.
    pub player: PlayerNumber,
}

impl Action for Unpenalize {
    fn execute(&self, game: &mut Game, _params: &Params) {
        game.teams[self.side][self.player].penalty_timer = Timer::Stopped;
        game.teams[self.side][self.player].penalty = Penalty::NoPenalty;
    }

    fn is_legal(&self, game: &Game, _params: &Params) -> bool {
        game.teams[self.side][self.player].penalty != Penalty::NoPenalty
            && game.teams[self.side][self.player].penalty != Penalty::Substitute
            && (game.teams[self.side][self.player]
                .penalty_timer
                .get_remaining()
                .is_zero()
                // We allow motion in Set penalties to be revoked while still in Set.
                || (game.teams[self.side][self.player].penalty == Penalty::MotionInSet
                    && game.state == State::Set))
    }
}
