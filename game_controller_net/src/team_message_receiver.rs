use std::{
    cmp::min,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

use anyhow::Result;
use bytes::Bytes;
use tokio::{net::UdpSocket, sync::mpsc};

use game_controller_msgs::{TEAM_MESSAGE_MAX_SIZE, TEAM_MESSAGE_PORT_BASE};

use crate::Event;

/// This function runs a receiver for team messages. The messages are UDP packets of a given
/// maximum length. It listens on any local address, but by specifying a local address, the caller
/// can choose between IPv4 and IPv6. The given team determines the UDP port on which messages are
/// expected. Received messages are passed to the caller as events in a [tokio::sync::mpsc]
/// channel.
pub async fn run_team_message_receiver(
    address: IpAddr,
    multicast: bool,
    team: u8,
    event_sender: mpsc::UnboundedSender<Event>,
) -> Result<()> {
    // Since we want to catch team messages that are too long, we expect one more byte than the
    // maximum message length.
    let mut buffer = vec![0u8; TEAM_MESSAGE_MAX_SIZE + 1];
    let socket = UdpSocket::bind((
        match address {
            IpAddr::V4(_) => IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            IpAddr::V6(_) => IpAddr::V6(Ipv6Addr::UNSPECIFIED),
        },
        TEAM_MESSAGE_PORT_BASE + (team as u16),
    ))
    .await?;
    if multicast {
        if let IpAddr::V4(address_v4) = address {
            let _ = socket.join_multicast_v4(Ipv4Addr::new(239, 0, 0, 1), address_v4);
        }
    }
    loop {
        let (length, address) = socket.recv_from(&mut buffer).await?;
        event_sender.send(Event::TeamMessage {
            host: address.ip(),
            team,
            data: Bytes::copy_from_slice(&buffer[0..min(length, TEAM_MESSAGE_MAX_SIZE)]),
            too_long: length > TEAM_MESSAGE_MAX_SIZE,
        })?;
    }
}
