use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{Game, Params, Phase, Side, State};

/// This struct defines an action which corresponds to the referee call "Set" in a penalty
/// shoot-out.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WaitForPenaltyShot;

impl Action for WaitForPenaltyShot {
    fn execute(&self, game: &mut Game, params: &Params) {
        if game.state != State::Initial && game.state != State::Timeout {
            game.sides = -game.sides;
            game.kicking_side = -game.kicking_side;
        }
        game.state = State::Set;
        game.primary_timer = Timer::Started {
            remaining: params.competition.penalty_shot_duration.try_into().unwrap(),
            run_condition: RunCondition::Playing,
            behavior_at_zero: BehaviorAtZero::Overflow,
        };
        game.secondary_timer = Timer::Stopped; // This can be set from a previous timeout.
        game.teams[game.kicking_side].penalty_shot += 1;
    }

    fn is_legal(&self, game: &Game, params: &Params) -> bool {
        game.phase == Phase::PenaltyShootout
            && (game.state == State::Initial
                || game.state == State::Timeout
                || (game.state == State::Finished && {
                    // At this point, kicking_side is the team that just finished its shot, so
                    // -kicking_side is the team that would have the next shot. The following
                    // should answer the question: Should that team still have a shot or not?
                    let score_difference = (game.teams[game.kicking_side].score as i16)
                        - (game.teams[-game.kicking_side].score as i16);
                    if game.teams[-game.kicking_side].penalty_shot
                        < params.competition.penalty_shots
                    {
                        // We are still in the regular penalty shoot-out. The following should
                        // answer if still both teams can win.

                        // How many shots does the next team still have in the regular shoot-out?
                        // (is at least 1)
                        let remaining_for_next = params.competition.penalty_shots
                            - game.teams[-game.kicking_side].penalty_shot;

                        // How many shots does the last team still have? (can be 0)
                        let remaining_for_last = params.competition.penalty_shots
                            - game.teams[game.kicking_side].penalty_shot;

                        // Can the next team still equalize?
                        score_difference <= remaining_for_next.into()
                            // Can the last team still equalize?
                            && -score_difference <= remaining_for_last.into()
                    } else if game.teams[-game.kicking_side].penalty_shot
                        < params.competition.penalty_shots
                            + params.competition.sudden_death_penalty_shots
                    {
                        // This means that the next shot will/would be part of the sudden death
                        // penalty shoot-out. The away team always gets another shot, but the
                        // home team only continues if the score is still even. At this point,
                        // there are other criteria to finish the game even if neither team
                        // scored, but that must be sorted out by the referee.
                        game.kicking_side == Side::Home || score_difference == 0
                    } else {
                        false
                    }
                }))
    }
}