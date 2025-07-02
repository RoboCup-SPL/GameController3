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
    mut hold_receiver: mpsc::UnboundedReceiver<bool>,
    shutdown_token: CancellationToken,
) -> Result<()> {
    let mut the_mute = false;
    let mut the_hold = false;
    let mut held_messages = vec![];

    loop {
        select! {
            message_or_error = action_ttsmsg_receiver.recv() => {
                if let Some(message_or_none) = message_or_error {
                    if let Some(message) = message_or_none {
                        println!("Received: {}", message);
                        if the_mute {
                            println!("But am mute");
                        }
                        else if the_hold {
                            println!("Holding it");
                            held_messages.push(message);
                        }
                        else {
                            naive_say(message);
                        }
                    }
                }
            },
            mute_or_error = mute_receiver.recv() => {
                if let Some(mute) = mute_or_error {
                    println!("Set mute to {}", mute);
                    the_mute = mute;
                    if mute {
                        // have mute double as "hold-cancel"
                        held_messages.clear();
                    }
                }
            },
            hold_or_error = hold_receiver.recv() => {
                if let Some(hold) = hold_or_error {
                    println!("Set hold to {}", hold);
                    the_hold = hold;
                    if !hold {
                        // release all held messages
                        for i in 0..held_messages.len() {
                            naive_say(held_messages[i].clone());
                        }
                        held_messages.clear();
                    }
                }
            },
            _ = shutdown_token.cancelled() => {
                return Ok(());
            },
        };
    }

}
