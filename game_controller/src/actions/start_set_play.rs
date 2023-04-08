use serde::{Deserialize, Serialize};

use crate::action::{Action, VAction};
use crate::actions::{FinishSetPlay, WaitForSetPlay};
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{Game, Params, Phase, SetPlay, Side, State};

/// This struct defines an action to start a set play. Depending on the set play type, this means
/// switching to the Ready state or just setting a flag for the current set play within the Playing
/// state.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartSetPlay {
    /// The side which can execute the set play.
    pub side: Side,
    /// The type of set play to start.
    pub set_play: SetPlay,
}

impl Action for StartSetPlay {
    fn execute(&self, game: &mut Game, params: &Params) {
        // Cancel all penalty timers if starting a set play from any state other than Playing.
        if game.state != State::Playing {
            game.teams.values_mut().for_each(|team| {
                team.players.iter_mut().for_each(|player| {
                    player.penalty_timer = Timer::Stopped;
                })
            });
        }

        if !params.competition.set_plays[self.set_play]
            .ready_duration
            .is_zero()
        {
            game.secondary_timer = Timer::Started {
                remaining: params.competition.set_plays[self.set_play]
                    .ready_duration
                    .try_into()
                    .unwrap(),
                run_condition: RunCondition::Always,
                // Automatically transition to the Set state when the timer expires.
                behavior_at_zero: BehaviorAtZero::Expire(vec![VAction::WaitForSetPlay(
                    WaitForSetPlay,
                )]),
            };
            game.state = State::Ready;
        } else {
            game.secondary_timer = Timer::Started {
                remaining: params.competition.set_plays[self.set_play]
                    .duration
                    .try_into()
                    .unwrap(),
                run_condition: RunCondition::Always,
                // Automatically deactivate the set play when the timer expires.
                behavior_at_zero: BehaviorAtZero::Expire(vec![VAction::FinishSetPlay(
                    FinishSetPlay,
                )]),
            };
        }
        game.set_play = self.set_play;
        game.kicking_side = self.side;
    }

    fn is_legal(&self, game: &Game) -> bool {
        self.set_play != SetPlay::NoSetPlay
            && game.phase != Phase::PenaltyShootout
            && (if self.set_play == SetPlay::KickOff {
                // For kick-offs, the kicking side is pre-filled so that only that team can take
                // the kick-off.
                (game.state == State::Initial || game.state == State::Timeout)
                    && game.kicking_side == self.side
            } else {
                // It must be Playing, and we can only start set play during other set plays if
                // they are for the other team (this is a shortcut, because FinishSetPlay should
                // have been clicked before).
                game.state == State::Playing
                    && (game.set_play == SetPlay::NoSetPlay || game.kicking_side != self.side)
            })
    }
}
