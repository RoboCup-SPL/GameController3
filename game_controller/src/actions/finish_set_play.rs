use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::timer::Timer;
use crate::types::{Game, Params, SetPlay, State};

/// This struct defines an action which corresponds to the referee call "Ball Free". It is the last
/// part of a set play (i.e. fourth part of "complex" set plays with Ready and Set state and second
/// part of "simple" set plays).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FinishSetPlay;

impl Action for FinishSetPlay {
    fn execute(&self, game: &mut Game, _params: &Params) {
        game.secondary_timer = Timer::Stopped;
        game.set_play = SetPlay::NoSetPlay;
    }

    fn is_legal(&self, game: &Game) -> bool {
        game.state == State::Playing && game.set_play != SetPlay::NoSetPlay
    }
}
