/**
 * Author: Francesco Petri <francesco.petri@uniroma1.it>
 * 
 * This file implements the text-to-speech module.
 * A whole tokio async module is probably a little overkill for how naive
 * and frontend-assisted this ended up being, but still, this modular structure
 * should allow a more Rust-savvy contributor greater freedom to apply
 * further improvements, as opposed to cramming this in the game_controller_core.
 * For example, an explicit message queue we have full control over
 * (as opposed to relying on the system's by passing interrupt:false)
 * would improve HOLD mode, but handling the callback interface offered
 * by the TTS crate proved too tricky for my novice skills.
 */

use tts::*;
use tokio::{select, sync::mpsc,};
use tokio_util::sync::CancellationToken;
use anyhow::{Result, anyhow};
use std::collections::HashMap;

pub fn get_voices() -> HashMap<String, Vec<String>> {
    let tts_voices = match Tts::default() {
        Ok(the_tts) => the_tts.voices().unwrap_or(vec![]),
        Err(_) => vec![],
    };
    let mut voices: HashMap<String, Vec<String>> = HashMap::new();
    for v in tts_voices {
        let lang = v.language().as_str().to_string();
        if !voices.contains_key(&lang) {
            voices.insert(lang.clone(), vec![]);
        }
        let x = voices.get_mut(&lang).unwrap();
        x.push(v.id());
    }
    for (_, x) in voices.iter_mut() {
        x.sort();
    }
    voices
}


fn naive_say(the_tts: &mut Tts, message: &String) {
    println!("Saying: {}", message);
    let _ = the_tts.speak(message, false);
}

pub async fn tts_event_loop(
    mut action_ttsmsg_receiver: mpsc::UnboundedReceiver<Option<String>>,
    initial_enabled: bool,
    initial_voice: String,
    mut mute_receiver: mpsc::UnboundedReceiver<bool>,
    mut hold_receiver: mpsc::UnboundedReceiver<bool>,
    shutdown_token: CancellationToken,
) -> Result<()> {
    if let Ok(mut the_tts) = Tts::default() {
        let mut the_mute = !initial_enabled;
        let mut the_hold = false;
        let mut held_messages = vec![];

        for v in the_tts.voices().unwrap() {
            if v.id() == initial_voice {
                let _ = the_tts.set_voice(&v);
            }
        }
    
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
                                naive_say(&mut the_tts, &message);
                            }
                        }
                    }
                },
                mute_or_error = mute_receiver.recv() => {
                    if let Some(mute) = mute_or_error {
                        println!("Set mute to {}", mute);
                        the_mute = mute;
                        if mute {
                            let _ = the_tts.stop();
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
                                naive_say(&mut the_tts, &held_messages[i]);
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
    else {
        println!("TTS initialization error, will just do without this time");
        return Err(anyhow!("TTS initialization error"));
    }

}
