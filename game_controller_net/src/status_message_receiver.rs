use std::{cmp::min, net::IpAddr};

use anyhow::Result;
use bytes::Bytes;
use tokio::{net::UdpSocket, sync::mpsc};

use game_controller_msgs::{STATUS_MESSAGE_PORT, STATUS_MESSAGE_SIZE};

use crate::Event;

/// This function runs a receiver for status messages. Status messages are UDP packets with a fixed
/// format, although that format isn't checked here. It listens only on the given local address.
/// Received messages are passed to the caller as events in a [tokio::sync::mpsc] channel.
pub async fn run_status_message_receiver(
    address: IpAddr,
    event_sender: mpsc::UnboundedSender<Event>,
) -> Result<()> {
    let mut buffer = vec![0u8; STATUS_MESSAGE_SIZE + 1];
    let socket = UdpSocket::bind((address, STATUS_MESSAGE_PORT)).await?;
    loop {
        let (length, address) = socket.recv_from(&mut buffer).await?;
        event_sender.send(Event::StatusMessage {
            host: address.ip(),
            data: Bytes::copy_from_slice(&buffer[0..min(length, STATUS_MESSAGE_SIZE)]),
            too_long: length > STATUS_MESSAGE_SIZE,
        })?;
    }
}
