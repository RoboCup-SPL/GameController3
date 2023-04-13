use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::timer::Timer;
use crate::types::{Game, Params, SetPlay, State};

/// This struct defines an action which corresponds to the referee call "Set". It is the second
/// part of "complex" set plays which have a Ready and Set state.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WaitForSetPlay;

impl Action for WaitForSetPlay {
    fn execute(&self, game: &mut Game, _params: &Params) {
        game.secondary_timer = Timer::Stopped;
        game.state = State::Set;
    }

    fn is_legal(&self, game: &Game, _params: &Params) -> bool {
        game.state == State::Ready && game.set_play != SetPlay::NoSetPlay
    }
}
