use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::action::{Action, VAction};
use crate::actions::{StartSetPlay, Unpenalize};
use crate::timer::{BehaviorAtZero, RunCondition, SignedDuration, Timer};
use crate::types::{Game, Params, Penalty, PenaltyCall, PlayerNumber, SetPlay, Side, State};

/// This struct defines an action to apply a penalty to players.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Penalize {
    /// The side whose player is penalized.
    pub side: Side,
    /// The number of the player who is penalized.
    pub player: PlayerNumber,
    /// The penalty which has been called for the player.
    pub call: PenaltyCall,
}

impl Action for Penalize {
    fn execute(&self, game: &mut Game, params: &Params) {
        // Map the penalty call to a penalty.
        let penalty = match self.call {
            PenaltyCall::RequestForPickUp => Penalty::PickedUp,
            PenaltyCall::IllegalPosition => {
                if game.state == State::Set {
                    Penalty::IllegalPositionInSet
                } else {
                    Penalty::IllegalPosition
                }
            }
            PenaltyCall::MotionInSet => Penalty::MotionInSet,
            PenaltyCall::FallenInactive => Penalty::FallenInactive,
            PenaltyCall::LocalGameStuck => Penalty::LocalGameStuck,
            PenaltyCall::BallHolding => Penalty::BallHolding,
            PenaltyCall::PlayerStance => Penalty::PlayerStance,
            PenaltyCall::Pushing => Penalty::PlayerPushing,
            PenaltyCall::Foul => Penalty::PlayerPushing,
            PenaltyCall::PenaltyKick => Penalty::PlayerPushing,
            PenaltyCall::PlayingWithArmsHands => Penalty::PlayingWithArmsHands,
            PenaltyCall::LeavingTheField => Penalty::LeavingTheField,
        };

        game.teams[self.side][self.player].penalty_timer = if penalty == Penalty::PickedUp
            && matches!(
                game.state,
                State::Initial | State::Finished | State::Timeout
            ) {
            // Picking up a player does not start a timer in "halted" game states.
            Timer::Stopped
        } else {
            Timer::Started {
                remaining: ({
                    // The duration is composed of the base duration plus the increment for each
                    // previous incremental penalty of this team.
                    let duration = params.competition.penalties[penalty].duration
                        + if params.competition.penalties[penalty].incremental {
                            params.competition.penalty_duration_increment
                                * game.teams[self.side].penalty_counter
                        } else {
                            Duration::ZERO
                        };
                    let previous_penalty = game.teams[self.side][self.player].penalty;
                    if penalty == Penalty::PickedUp && previous_penalty != Penalty::NoPenalty {
                        // Picking up a player in other states should keep the previous timer if
                        // the player was  already penalized, but enforce that the total penalty
                        // time is at least that of the pick-up penalty.
                        let extra_penalty_duration = TryInto::<SignedDuration>::try_into(duration)
                            .unwrap()
                            - params.competition.penalties[previous_penalty].duration;
                        game.teams[self.side][self.player]
                            .penalty_timer
                            .get_remaining()
                            + if extra_penalty_duration.is_positive() {
                                // If any penalty that is shorter than pick-up is incremental, we
                                // would have to save how long the duration *actually* was. I
                                // don't want to introduce extra complexity only for this special
                                // case as long as it isn't necessary.
                                assert!(
                                    !params.competition.penalties[previous_penalty].incremental
                                );
                                extra_penalty_duration
                            } else {
                                SignedDuration::ZERO
                            }
                    } else {
                        duration.try_into().unwrap()
                    }
                }),
                run_condition: RunCondition::ReadyOrPlaying,
                // Motion in Set is removed automatically.
                behavior_at_zero: if penalty == Penalty::MotionInSet {
                    BehaviorAtZero::Expire(vec![VAction::Unpenalize(Unpenalize {
                        side: self.side,
                        player: self.player,
                    })])
                } else {
                    BehaviorAtZero::Clip
                },
            }
        };

        game.teams[self.side][self.player].penalty = penalty;
        if params.competition.penalties[penalty].incremental {
            game.teams[self.side].penalty_counter += 1;
        }

        // If this call requires switching to a set play, it is started here.
        if let Some(set_play) = match self.call {
            PenaltyCall::Foul => Some(SetPlay::PushingFreeKick),
            PenaltyCall::PenaltyKick => Some(SetPlay::PenaltyKick),
            _ => None,
        } {
            StartSetPlay {
                side: -self.side,
                set_play,
            }
            .execute(game, params);
        }
    }

    fn is_legal(&self, game: &Game) -> bool {
        (game.teams[self.side][self.player].penalty == Penalty::NoPenalty
            || (self.call == PenaltyCall::RequestForPickUp
                && game.teams[self.side][self.player].penalty != Penalty::PickedUp))
            && (match self.call {
                PenaltyCall::RequestForPickUp => true,
                PenaltyCall::IllegalPosition => {
                    game.state == State::Set || game.state == State::Playing
                }
                PenaltyCall::MotionInSet => game.state == State::Set,
                PenaltyCall::FallenInactive => {
                    game.state == State::Ready
                        || game.state == State::Set
                        || game.state == State::Playing
                }
                PenaltyCall::LocalGameStuck => game.state == State::Playing,
                PenaltyCall::BallHolding => game.state == State::Playing,
                PenaltyCall::PlayerStance => {
                    game.state == State::Ready
                        || game.state == State::Set
                        || game.state == State::Playing
                }
                PenaltyCall::Pushing => game.state == State::Ready || game.state == State::Playing,
                PenaltyCall::Foul => {
                    game.state == State::Playing && game.set_play == SetPlay::NoSetPlay
                }
                PenaltyCall::PenaltyKick => {
                    game.state == State::Playing && game.set_play == SetPlay::NoSetPlay
                }
                PenaltyCall::PlayingWithArmsHands => game.state == State::Playing,
                PenaltyCall::LeavingTheField => {
                    game.state == State::Ready || game.state == State::Playing
                }
            })
    }
}
