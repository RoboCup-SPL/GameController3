use std::mem::replace;

use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{Game, Params, Penalty, PlayerNumber, Side, State};

/// This struct defines an action to substitute players.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Substitute {
    /// The side which does the substitution.
    pub side: Side,
    /// The player who comes in (currently a substitute).
    pub player_in: PlayerNumber,
    /// The player who comes off (will become a substitute).
    pub player_out: PlayerNumber,
}

impl Action for Substitute {
    fn execute(&self, game: &mut Game, params: &Params) {
        if game.teams[self.side][self.player_out].penalty == Penalty::NoPenalty
            && matches!(game.state, State::Ready | State::Set | State::Playing)
        {
            // Players that are substituted while not being penalized must still wait as if they
            // were picked-up.
            assert!(!params.competition.penalties[Penalty::PickedUp].incremental);
            game.teams[self.side][self.player_in].penalty = Penalty::PickedUp;
            game.teams[self.side][self.player_in].penalty_timer = Timer::Started {
                remaining: params.competition.penalties[Penalty::PickedUp]
                    .duration
                    .try_into()
                    .unwrap(),
                run_condition: RunCondition::ReadyOrPlaying,
                behavior_at_zero: BehaviorAtZero::Clip,
            };
            game.teams[self.side][self.player_out].penalty_timer = Timer::Stopped;
        } else {
            // Inherit the penalty and the timer.
            game.teams[self.side][self.player_in].penalty =
                game.teams[self.side][self.player_out].penalty;
            game.teams[self.side][self.player_in].penalty_timer = replace(
                &mut game.teams[self.side][self.player_out].penalty_timer,
                Timer::Stopped,
            );
        }
        game.teams[self.side][self.player_out].penalty = Penalty::Substitute;
        if game.teams[self.side].goalkeeper == self.player_out {
            game.teams[self.side].goalkeeper = self.player_in;
        }
    }

    fn is_legal(&self, game: &Game, _params: &Params) -> bool {
        self.player_in != self.player_out
            && game.teams[self.side][self.player_in].penalty == Penalty::Substitute
            && game.teams[self.side][self.player_out].penalty != Penalty::Substitute
    }
}
