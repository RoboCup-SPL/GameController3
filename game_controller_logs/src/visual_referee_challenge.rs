//! This module implements functions to evaluate the visual referee challenge (RoboCup 2023).

use anyhow::{bail, Result};
use bytes::Bytes;

use game_controller_core::{
    log::{LogEntry, LoggedMetadata, TimestampedLogEntry},
    types::Params,
};
use game_controller_msgs::VrcMessage;

/// This function extracts the reported signals for the visual referee challenge from a single
/// game. For each valid report, a line is written to the standard output with N comma separated
/// values: the time in milliseconds since the log was started, the team number of the reporting
/// player, the player number of the reporting player, and the reported gesture.
pub fn evaluate(entries: Vec<TimestampedLogEntry>) -> Result<()> {
    let mut iter = entries.iter();
    let metadata: &LoggedMetadata =
        if let LogEntry::Metadata(metadata) = &iter.next().unwrap().entry {
            metadata
        } else {
            bail!("first log entry must be metadata");
        };
    let params: &Params = &metadata.params;
    iter.for_each(|entry| {
        match &entry.entry {
            LogEntry::StatusMessage(status_message) => {
                if let Ok(vrc_message) =
                    VrcMessage::try_from(Bytes::from(status_message.data.clone()))
                {
                    if let Some(_side) = params.game.get_side(vrc_message.team_number) {
                        println!(
                            "{},{},{},{}",
                            entry.timestamp.as_millis(),
                            vrc_message.team_number,
                            vrc_message.player_number,
                            vrc_message.gesture,
                        );
                    }
                }
            }
            _ => {}
        }
    });
    Ok(())
}
