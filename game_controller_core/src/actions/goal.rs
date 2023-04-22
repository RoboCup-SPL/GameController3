use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::actions::StartSetPlay;
use crate::timer::Timer;
use crate::types::{Phase, SetPlay, Side, State};

/// This struct defines an action for when a goal has been scored.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    /// The side which has scored a goal.
    pub side: Side,
}

impl Action for Goal {
    fn execute(&self, c: &mut ActionContext) {
        if c.game.phase != Phase::PenaltyShootout
            && !c.fork(c.params.competition.delay_after_goal, |_| false)
        {
            return;
        }

        c.game.secondary_timer = Timer::Stopped;
        c.game.set_play = SetPlay::NoSetPlay;

        if !c.game.teams[self.side].illegal_communication {
            c.game.teams[self.side].score += 1;
        }
        if c.game.phase != Phase::PenaltyShootout {
            if c.game.teams[self.side].score
                >= c.game.teams[-self.side].score + c.params.competition.mercy_rule_score_difference
            {
                // Mercy rule: At a certain goal difference, the game is finished.
                c.game.teams.values_mut().for_each(|team| {
                    team.players.iter_mut().for_each(|player| {
                        player.penalty_timer = Timer::Stopped;
                    })
                });
                c.game.phase = Phase::SecondHalf;
                c.game.state = State::Finished;
            } else {
                // A kick-off for the other team.
                StartSetPlay {
                    side: -self.side,
                    set_play: SetPlay::KickOff,
                }
                .execute(c);
            }
        } else {
            c.game.teams[self.side].penalty_shot_mask |=
                1u16 << (c.game.teams[self.side].penalty_shot - 1);
            c.game.state = State::Finished;
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.state == State::Playing
            && (c.game.phase != Phase::PenaltyShootout || self.side == c.game.kicking_side)
    }
}
