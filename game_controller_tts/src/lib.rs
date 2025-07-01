/**
 * Author: Francesco Petri <francesco.petri@uniroma1.it>
 * 
 * This file implements the text-to-speech module.
 * A whole tokio async module is probably a little overkill for how naive
 * and frontend-assisted this ended up being, but still, this modular structure
 * should allow a more Rust-savvy contributor greater freedom to apply
 * further improvements, as opposed to cramming this in the game_controller_core.
 * For example, an explicit message queue we have full control over
 * would improve HOLD mode, but handling the callback interface offered
 * by the TTS crate proved too tricky for my novice skills.
 * Or maybe, as a minor change, one may want to try a different TTS library:
 * this is as easy as replacing naive_say with whatever functiion you can write.
 */

use tts::*;
use tokio::{select, sync::mpsc,};
use tokio_util::sync::CancellationToken;
use anyhow::Result;

pub fn naive_say(message: String) {
    println!("Saying: {}", message);
    let mut the_tts: Tts = Tts::default().unwrap();  // TODO error handling here
    let _ = the_tts.speak(message, false);
}

pub async fn tts_event_loop(
    mut action_ttsmsg_receiver: mpsc::UnboundedReceiver<Option<String>>,
    mut mute_receiver: mpsc::UnboundedReceiver<bool>,
    shutdown_token: CancellationToken,
) -> Result<()> {
    println!("zan zan zan");
    let mut the_mute = false;

    loop {
        select! {
            message_or_error = action_ttsmsg_receiver.recv() => {
                if let Some(message_or_none) = message_or_error {
                    if let Some(message) = message_or_none {
                        println!("Received: {}", message);
                        if the_mute {
                            println!("But am mute");
                        }
                        else {
                            naive_say(message);
                        }
                    }
                }
            },
            mute_or_error = mute_receiver.recv() => {
                if let Some(mute) = mute_or_error {
                    println!("Confirm setting mute to {}", mute);
                    the_mute = mute;
                }
            },
            _ = shutdown_token.cancelled() => {
                return Ok(());
            },
        };
    }

}
