//! This module defines common types for actions. Actions can be checked for legality against game
//! states and can be applied to game states. The action variant [VAction] implements the [Action]
//! trait and dispatches calls to the inner actions, which must implement the [Action] trait, too.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use trait_enum::trait_enum;

use crate::{
    actions::*,
    timer::{BehaviorAtZero, RunCondition, Timer},
    types::{Game, Params},
    DelayHandler,
};

/// This trait must be implemented by all actions in the [crate::actions] module.
pub trait Action {
    /// This function applies the action to the game state.
    fn execute(&self, c: &mut ActionContext);

    /// This function returns whether the action is legal in the given game state.
    fn is_legal(&self, c: &ActionContext) -> bool;
}

trait_enum! {
    /// This is a "variant" of all actions.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type", content = "args", rename_all = "camelCase")]
    pub enum VAction: Action {
        AddExtraTime,
        FinishHalf,
        FinishPenaltyShot,
        FinishSetPlay,
        FreePenaltyShot,
        FreeSetPlay,
        GlobalGameStuck,
        Goal,
        Penalize,
        SelectPenaltyShotPlayer,
        StartPenaltyShootout,
        StartSetPlay,
        Substitute,
        SwitchHalf,
        TeamMessage,
        Timeout,
        Undo,
        Unpenalize,
        WaitForPenaltyShot,
        WaitForSetPlay,
    }
}

/// This struct defines a context in which an action is evaluated.
pub struct ActionContext<'a> {
    /// The game state on which the action operates.
    pub game: &'a mut Game,
    /// The parameters which the action uses.
    pub params: &'a Params,
    delay: Option<&'a mut Option<DelayHandler>>,
}

impl ActionContext<'_> {
    /// This function creates a new action context.
    pub fn new<'a>(
        game: &'a mut Game,
        params: &'a Params,
        delay: Option<&'a mut Option<DelayHandler>>,
    ) -> ActionContext<'a> {
        ActionContext {
            game,
            params,
            delay,
        }
    }

    /// This function "forks" the game state. If the context is in a delayed game, it does nothing
    /// and returns [false]. If the context is in the true game, it forks a delayed game state that
    /// does not include the following effects of the current action and returns [true]. The
    /// delayed game state will be canceled after a given duration, or if any future action would
    /// be illegal in the delayed game state, unless it is accepted by the given ignore function.
    pub fn fork(
        &mut self,
        duration: Duration,
        ignore: impl Fn(&VAction) -> bool + Send + 'static,
    ) -> bool {
        if let Some(delay) = self.delay.as_mut() {
            **delay = Some(DelayHandler {
                game: self.game.clone(),
                timer: Timer::Started {
                    remaining: duration.try_into().unwrap(),
                    run_condition: RunCondition::Always,
                    behavior_at_zero: BehaviorAtZero::Expire(vec![]),
                },
                ignore: Box::new(ignore),
            });
            true
        } else {
            false
        }
    }
}
