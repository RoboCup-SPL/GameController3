use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::actions::{FinishHalf, StartSetPlay};
use crate::timer::Timer;
use crate::types::{Phase, SetPlay, Side, State};

/// This struct defines an action for when a goal has been scored.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    /// The side which has scored a goal.
    pub side: Side,
}

impl Action for Goal {
    fn execute(&self, c: &mut ActionContext) {
        // Mercy rule: At a certain goal difference, the game is finished.
        let mercy_rule = c.game.phase != Phase::PenaltyShootout
            && !c.game.teams[self.side].illegal_communication
            && (c.game.teams[self.side].score + 1)
                >= c.game.teams[-self.side].score
                    + c.params.competition.mercy_rule_score_difference;
        if !c.params.game.test.no_delay
            && c.game.phase != Phase::PenaltyShootout
            && c.params.competition.challenge_mode.is_none()
            && !mercy_rule
            && !c.fork(c.params.competition.delay_after_goal, |_| false)
        {
            return;
        }

        if !c.game.teams[self.side].illegal_communication {
            c.game.teams[self.side].score += 1;
        }
        if c.params.competition.challenge_mode.is_some() {
            return;
        }
        if mercy_rule {
            c.game.teams.values_mut().for_each(|team| {
                team.players.iter_mut().for_each(|player| {
                    player.penalty_timer = Timer::Stopped;
                })
            });
            c.game.phase = Phase::SecondHalf;
            FinishHalf.execute(c);
        } else if c.game.phase != Phase::PenaltyShootout {
            // A kick-off for the other team.
            StartSetPlay {
                side: Some(-self.side),
                set_play: SetPlay::KickOff,
            }
            .execute(c);
        } else {
            c.game.teams[self.side].penalty_shot_mask |=
                1u16 << (c.game.teams[self.side].penalty_shot - 1);
            c.game.state = State::Finished;
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.state == State::Playing
            && (c.game.phase != Phase::PenaltyShootout
                || c.game.kicking_side.is_none_or(|side| side == self.side))
            && (c.params.competition.challenge_mode.is_none()
                || self.side == c.params.game.kick_off_side)
    }

    fn get_tts_message(&self, c: &ActionContext) -> Option<String> {
        Some(format!(
            "Goal for {}",
            c.params.game.teams[self.side].field_player_color,
        ))
    }
}
