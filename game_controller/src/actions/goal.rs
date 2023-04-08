use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::actions::StartSetPlay;
use crate::timer::Timer;
use crate::types::{Game, Params, Phase, SetPlay, Side, State};

/// This struct defines an action for when a goal has been scored.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    /// The side which has scored a goal.
    pub side: Side,
}

impl Action for Goal {
    fn execute(&self, game: &mut Game, params: &Params) {
        game.secondary_timer = Timer::Stopped;
        game.set_play = SetPlay::NoSetPlay;

        if !game.teams[self.side].illegal_communication {
            game.teams[self.side].score += 1;
        }
        if game.phase != Phase::PenaltyShootout {
            if game.teams[self.side].score
                >= game.teams[-self.side].score + params.competition.mercy_rule_score_difference
            {
                // Mercy rule: At a certain goal difference, the game is finished.
                game.teams.values_mut().for_each(|team| {
                    team.players.iter_mut().for_each(|player| {
                        player.penalty_timer = Timer::Stopped;
                    })
                });
                game.phase = Phase::SecondHalf;
                game.state = State::Finished;
            } else {
                // A kick-off for the other team.
                StartSetPlay {
                    side: -self.side,
                    set_play: SetPlay::KickOff,
                }
                .execute(game, params);
            }
        } else {
            game.teams[self.side].penalty_shot_mask |=
                1u16 << (game.teams[self.side].penalty_shot - 1);
            game.state = State::Finished;
        }
    }

    fn is_legal(&self, game: &Game) -> bool {
        game.state == State::Playing
            && (game.phase != Phase::PenaltyShootout || self.side == game.kicking_side)
    }
}
