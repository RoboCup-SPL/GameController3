//! This crate contains network services for the GameController.

use std::net::IpAddr;

use bytes::Bytes;

mod control_message_sender;
mod monitor_request_receiver;
mod status_message_forwarder;
mod status_message_receiver;
mod team_message_receiver;

pub use control_message_sender::run_control_message_sender;
pub use monitor_request_receiver::run_monitor_request_receiver;
pub use status_message_forwarder::run_status_message_forwarder;
pub use status_message_receiver::run_status_message_receiver;
pub use team_message_receiver::run_team_message_receiver;

/// This enumerates network events.
#[derive(Debug)]
pub enum Event {
    /// An incoming monitor request.
    MonitorRequest {
        /// The host which sent the request.
        host: IpAddr,
        /// The payload of the request.
        data: Bytes,
        /// Whether there would have been more bytes in the request.
        too_long: bool,
    },
    /// An incoming status message (from a player).
    StatusMessage {
        /// The host which sent the message.
        host: IpAddr,
        /// The payload of the message.
        data: Bytes,
        /// Whether there would have been more bytes in the message.
        too_long: bool,
    },
    /// An incoming team message (from a player).
    TeamMessage {
        /// The host which sent the message.
        host: IpAddr,
        /// The team which sent the message.
        team: u8,
        /// The payload of the message.
        data: Bytes,
        /// Whether there would have been more bytes in the message.
        too_long: bool,
    },
}
