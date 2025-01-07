use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::actions::{FinishHalf, StartSetPlay};
use crate::timer::Timer;
use crate::types::{Phase, SetPlay, Side, State};

use tts::*;

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

        c.game.secondary_timer = Timer::Stopped;
        c.game.set_play = SetPlay::NoSetPlay;

        if !c.game.teams[self.side].illegal_communication {
            c.game.teams[self.side].score += 1;
        }
        if mercy_rule || c.params.competition.challenge_mode.is_some() {
            c.game.teams.values_mut().for_each(|team| {
                team.players.iter_mut().for_each(|player| {
                    player.penalty_timer = Timer::Stopped;
                })
            });
            if mercy_rule {
                c.game.phase = Phase::SecondHalf;
            }
            FinishHalf.execute(c);
        } else if c.game.phase != Phase::PenaltyShootout {
            // A kick-off for the other team.
            StartSetPlay {
                side: -self.side,
                set_play: SetPlay::KickOff,
            }
            .execute(c);
        } else {
            c.game.teams[self.side].penalty_shot_mask |=
                1u16 << (c.game.teams[self.side].penalty_shot - 1);
            c.game.state = State::Finished;
        }

        // Audio output
        let msg = format!("Goal for {}", c.params.game.teams[self.side].field_player_color);
        println!("{}", msg);
        let mut the_tts: Tts = Tts::default().unwrap();
        the_tts.speak(msg, false);
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.state == State::Playing
            && (c.game.phase != Phase::PenaltyShootout || self.side == c.game.kicking_side)
            && (c.params.competition.challenge_mode.is_none()
                || self.side
                    == (if c.game.phase == Phase::FirstHalf {
                        c.params.game.kick_off_side
                    } else {
                        -c.params.game.kick_off_side
                    }))
    }
}
