use serde::{Deserialize, Serialize};

use crate::action::{Action, ActionContext};
use crate::timer::Timer;
use crate::types::{Penalty, Phase, Player, PlayerNumber, Side, State};

/// This struct defines an action that switches between "normal mode" and "fallback mode".
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SwitchTeamMode {
    /// The side which switches its mode.
    pub side: Side,
}

impl Action for SwitchTeamMode {
    fn execute(&self, c: &mut ActionContext) {
        if let Some(players_per_team_fallback_mode) =
            c.params.competition.players_per_team_fallback_mode
        {
            if players_per_team_fallback_mode < c.params.competition.players_per_team {
                type SwitchParameters<'a> = (
                    Box<dyn FnMut(&(usize, &mut Player)) -> bool + 'a>,
                    Penalty,
                    u8,
                );
                let goalkeeper_index = c.game.teams[self.side]
                    .goalkeeper
                    .map(|goalkeeper| u8::from(goalkeeper) - PlayerNumber::MIN);
                let (predicate, penalty, target_players): SwitchParameters =
                    if !c.game.teams[self.side].fallback_mode {
                        (
                            Box::new(|(index, player)| {
                                player.penalty != Penalty::Substitute
                                    && goalkeeper_index
                                        .is_none_or(|goalkeeper| goalkeeper != (*index as u8))
                            }),
                            Penalty::Substitute,
                            players_per_team_fallback_mode,
                        )
                    } else {
                        (
                            Box::new(|(_, player)| player.penalty == Penalty::Substitute),
                            Penalty::NoPenalty,
                            c.params.competition.players_per_team,
                        )
                    };
                c.game.teams[self.side]
                    .players
                    .iter_mut()
                    .enumerate()
                    .filter(predicate)
                    .map(|(_, player)| player)
                    .take(
                        (c.params.competition.players_per_team - players_per_team_fallback_mode)
                            as usize,
                    )
                    .for_each(|player| {
                        player.penalty = penalty;
                        player.penalty_timer = Timer::Stopped;
                    });
                assert!(
                    c.game.teams[self.side]
                        .players
                        .iter()
                        .filter(|player| player.penalty != Penalty::Substitute)
                        .count()
                        == target_players.into()
                );
            }
            c.game.teams[self.side].fallback_mode = !c.game.teams[self.side].fallback_mode;
        }
    }

    fn is_legal(&self, c: &ActionContext) -> bool {
        c.params
            .competition
            .players_per_team_fallback_mode
            .is_some()
            && c.game.phase != Phase::PenaltyShootout
            && (c.game.state == State::Initial
                || (c.game.state == State::Timeout && !c.game.teams[self.side].fallback_mode))
    }
}
