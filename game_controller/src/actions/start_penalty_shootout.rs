use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::timer::Timer;
use crate::types::{Game, Params, Penalty, Phase, SetPlay, Side, SideMapping, State};

/// This struct defines an action which starts a penalty (kick) shoot-out. To disambiguate this
/// from penalty kicks as set plays within the game, penalty kicks in a penalty (kick) shoot-out
/// are mostly referred to as "penalty shots".
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartPenaltyShootout {
    /// This defines the goal on which all penalty shots are taken. Since the home team always has
    /// the first kick, [SideMapping::HomeDefendsLeftGoal] means that the right goal is used.
    pub sides: SideMapping,
}

impl Action for StartPenaltyShootout {
    fn execute(&self, game: &mut Game, _params: &Params) {
        // Make all players substitutes.
        game.teams.values_mut().for_each(|team| {
            team.penalty_shot = 0;
            team.penalty_shot_mask = 0;
            team.players.iter_mut().for_each(|player| {
                player.penalty = Penalty::Substitute;
                player.penalty_timer = Timer::Stopped;
            })
        });

        game.sides = self.sides;
        game.phase = Phase::PenaltyShootout;
        game.state = State::Initial;
        game.set_play = SetPlay::NoSetPlay;
        // "The first (left) team in the GameController will have the striker robot for the first
        // penalty kick." - 2023 rule book section 3.16
        game.kicking_side = Side::Home;
        game.primary_timer = Timer::Stopped;
        game.secondary_timer = Timer::Stopped;
    }

    fn is_legal(&self, game: &Game) -> bool {
        game.phase == Phase::SecondHalf
            && game.state == State::Finished
            && game.teams[Side::Home].score == game.teams[Side::Away].score
    }
}
