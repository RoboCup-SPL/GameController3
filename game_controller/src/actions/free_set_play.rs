use serde::{Deserialize, Serialize};

use crate::action::{Action, VAction};
use crate::actions::FinishSetPlay;
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{Game, Params, SetPlay, State};

/// This struct defines an action which corresponds to the referee call "Playing". It is the third
/// part of "complex" set plays which have a Ready and Set state.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FreeSetPlay;

impl Action for FreeSetPlay {
    fn execute(&self, game: &mut Game, params: &Params) {
        game.secondary_timer = Timer::Started {
            remaining: params.competition.set_plays[game.set_play]
                .duration
                .try_into()
                .unwrap(),
            run_condition: RunCondition::Always,
            behavior_at_zero: BehaviorAtZero::Expire(vec![VAction::FinishSetPlay(FinishSetPlay)]),
        };
        game.state = State::Playing;
    }

    fn is_legal(&self, game: &Game) -> bool {
        game.state == State::Set && game.set_play != SetPlay::NoSetPlay
    }
}
