//! This module defines common types for actions. Actions can be checked for legality against game
//! states and can be applied to game states. The action variant [VAction] implements the [Action]
//! trait and dispatches calls to the inner actions, which must implement the [Action] trait, too.

use serde::{Deserialize, Serialize};
use trait_enum::trait_enum;

use crate::actions::*;
use crate::types::{Game, Params};

/// This trait must be implemented by all actions in the [crate::actions] module.
pub trait Action {
    /// This function applies the action to the game state.
    fn execute(&self, game: &mut Game, params: &Params);

    /// This function returns whether the action is legal in the given game state.
    fn is_legal(&self, game: &Game) -> bool;
}

trait_enum! {
    /// This is a "variant" of all actions.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type", content = "args", rename_all = "camelCase")]
    pub enum VAction: Action {
        FinishHalf,
        FinishSetPlay,
        FreeSetPlay,
        GlobalGameStuck,
        Goal,
        Penalize,
        StartSetPlay,
        Substitute,
        SwitchHalf,
        TeamMessage,
        Timeout,
        Unpenalize,
        WaitForSetPlay,
    }
}
