use std::{cmp::min, net::IpAddr};

use anyhow::Result;
use bytes::Bytes;
use tokio::{net::UdpSocket, sync::mpsc};

use game_controller_msgs::{MONITOR_REQUEST_PORT, MONITOR_REQUEST_SIZE};

use crate::Event;

/// This function runs a receiver for monitor requests. It listens only on the given local address.
/// Received messages are passed to the caller as events in a [tokio::sync::mpsc] channel.
pub async fn run_monitor_request_receiver(
    address: IpAddr,
    event_sender: mpsc::UnboundedSender<Event>,
) -> Result<()> {
    let mut buffer = vec![0u8; MONITOR_REQUEST_SIZE + 1];
    let socket = UdpSocket::bind((address, MONITOR_REQUEST_PORT)).await?;
    loop {
        let (length, address) = socket.recv_from(&mut buffer).await?;
        event_sender.send(Event::MonitorRequest {
            host: address.ip(),
            data: Bytes::copy_from_slice(&buffer[0..min(length, MONITOR_REQUEST_SIZE)]),
            too_long: length > MONITOR_REQUEST_SIZE,
        })?;
    }
}
