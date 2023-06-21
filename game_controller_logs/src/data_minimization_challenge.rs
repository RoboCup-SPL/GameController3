//! This module implements functions to evaluate the data minimization challenge (RoboCup 2023).

use std::time::Duration;

use anyhow::{bail, Result};
use enum_map::enum_map;

use game_controller_core::{
    log::{LogEntry, LoggedMetadata, TimestampedLogEntry},
    types::{Game, Params, Penalty, Phase, Side, State},
};

/// This function checks if the given game is in a state where team messages are counted for this
/// challenge.
fn is_valid_state(game: &Game) -> bool {
    game.phase != Phase::PenaltyShootout
        && matches!(game.state, State::Ready | State::Playing | State::Set)
}

/// This function evaluates the statistics for the data minimization challenge on a single game.
/// For each team, a line is written to the standard output with three comma separated values: the
/// team number, the number of payload bytes that the team sent during the game, and the overall
/// uptime of the team during the game.
pub fn evaluate(entries: Vec<TimestampedLogEntry>) -> Result<()> {
    let mut iter = entries.iter();
    let metadata: &LoggedMetadata =
        if let LogEntry::Metadata(metadata) = &iter.next().unwrap().entry {
            metadata
        } else {
            bail!("first log entry must be metadata");
        };
    let params: &Params = &metadata.params;
    let mut stats = enum_map! {
        _ => (0usize, Duration::ZERO),
    };
    let mut last: Option<(&Game, Duration)> = None;
    for entry in iter {
        match &entry.entry {
            LogEntry::GameState(state) => {
                if let Some((last_state, last_timestamp)) = last {
                    if is_valid_state(last_state) {
                        let dt = entry.timestamp - last_timestamp;
                        for side in [Side::Home, Side::Away] {
                            let active_players = last_state.teams[side]
                                .players
                                .iter()
                                .filter(|player| player.penalty == Penalty::NoPenalty)
                                .count() as u32;
                            stats[side].1 += dt * active_players;
                        }
                    }
                }
                last = Some((state, entry.timestamp));
            }
            LogEntry::TeamMessage(team_message) => {
                if let Some((last_state, _)) = last {
                    if is_valid_state(last_state) {
                        if let Some(side) = params.game.get_side(team_message.team) {
                            stats[side].0 += team_message.data.len();
                        }
                    }
                }
            }
            _ => {}
        }
    }
    for side in [Side::Home, Side::Away] {
        println!(
            "{},{},{}",
            params.game.teams[side].number,
            stats[side].0,
            stats[side].1.as_secs()
        );
    }
    Ok(())
}
