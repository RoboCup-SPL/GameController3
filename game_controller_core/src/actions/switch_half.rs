use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::timer::{BehaviorAtZero, RunCondition, Timer};
use crate::types::{Penalty, Phase, PlayerNumber, State};

/// This struct defines an action that switches from the end of the first half to the beginning of
/// the second half, including the switch of sides.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SwitchHalf;

impl Action for SwitchHalf {
    fn execute(&self, c: &mut ActionContext) {
        // Unpenalize all players that are not substitutes. Maybe picked up players should stay
        // picked up, but the old GameController unpenalized them, too.
        c.game.teams.values_mut().for_each(|team| {
            team.players
                .iter_mut()
                .filter(|player| player.penalty != Penalty::Substitute)
                .for_each(|player| {
                    player.penalty = Penalty::NoPenalty;
                    player.penalty_timer = Timer::Stopped;
                })
        });

        if c.params.competition.challenge_mode.is_some() {
            c.game.teams[c.params.game.kick_off_side].goalkeeper = c.game.teams
                [c.params.game.kick_off_side]
                .players
                .iter()
                .enumerate()
                .find(|player| player.1.penalty != Penalty::Substitute)
                .map(|player| PlayerNumber::new((player.0 as u8) + PlayerNumber::MIN));
            c.game.teams[-c.params.game.kick_off_side].goalkeeper = None;
        }

        c.game.sides = -c.params.game.side_mapping;
        c.game.phase = Phase::SecondHalf;
        c.game.state = State::Initial;
        c.game.kicking_side = Some(-c.params.game.kick_off_side);

        c.game.primary_timer = Timer::Started {
            remaining: c.params.competition.half_duration.try_into().unwrap(),
            run_condition: RunCondition::MainTimer,
            behavior_at_zero: BehaviorAtZero::Overflow,
        };
        c.game.switch_half_timer = Timer::Stopped;
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.game.phase == Phase::FirstHalf && c.game.state == State::Finished
    }
}
