use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::timer::Timer;
use crate::types::{Penalty, SetPlay, State};

/// This struct defines an action which corresponds to the referee call "Set". It is the second
/// part of "complex" set plays which have a Ready and Set state.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WaitForSetPlay;

impl Action for WaitForSetPlay {
    fn execute(&self, c: &mut ActionContext) {
        c.game.teams.values_mut().for_each(|team| {
            team.players
                .iter_mut()
                .filter(|player| player.penalty == Penalty::MotionInStandby)
                .for_each(|player| {
                    player.penalty = Penalty::NoPenalty;
                    player.penalty_timer = Timer::Stopped;
                })
        });

        c.game.secondary_timer = Timer::Stopped;
        c.game.state = State::Set;
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.state == State::Ready && c.game.set_play != SetPlay::NoSetPlay
    }
}
