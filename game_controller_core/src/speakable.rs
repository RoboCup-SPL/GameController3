//! This module defines a trait implemented by all actions that can be spoken aloud,
//! such as set plays.

use crate::action::ActionContext;

pub trait Speakable {
    /// This function speaks the message corresponding to the action.
    fn speak(&self, c: &mut ActionContext);
}

// Project notes:
// Maybe use one global object that receives all TTS requests and
// handles them asynchronously in an event-driven way.
// I assume Rust can handle such a paradigm (which sounds singleton-like,
// now that I think about it), the question is how.
// 
// This thing would have a QUEUE to store the messages still to be spoken (as strings)
// and the following events might happen to it at any time:
// - Receive new TTS request:
//   Called at the end of the speak() function of each Speakable action.
//   Adds the message at the end of the QUEUE and ADVANCES THE QUEUE,
//   but only if the MUTE flag is off.
// - Finished speaking a message:
//   The tts crate has on_utterance_begin, on_utterance_end, on_utterance_stop
//   that can be used for this purpose, but they're not supported on all backends*.
//   This event simply ADVANCES THE QUEUE.
// - Spacebar pressed:
//   Called by whatever built-in method or crate Rust has to handle keyboard input.
//   Turns the HOLD flag on.
// - Spacebar released:
//   Called by whatever built-in method or crate Rust has to handle keyboard input.
//   Turns the HOLD flag off and ADVANCES THE QUEUE.
// - Toggle mute on:
//   Called by whatever built-in method or crate Rust has to handle keyboard input.
//   Turns the MUTE flag on and empties the QUEUE.
// - Toggle mute off:
//   Called by whatever built-in method or crate Rust has to handle keyboard input.
//   Turns the MUTE flag off.
// 
// ADVANCING THE QUEUE means the following:
// - If either the HOLD or MUTE flag is on, do nothing.
// - If a message is currently being spoken, do nothing
//   (The tts crate has is_speaking, but it appears it's not supported by all backends*.
//    If is_speaking is not an option, will need to write something
//    clever use of on_utterance_begin, on_utterance_end, on_utterance_stop,
//    assuming it is possible for them to be supported where is_speaking is not.)
// - If the QUEUE is empty, do nothing.
// - If none of the above holds, read the first element of the QUEUE,
//   delete it from the QUEUE, and begin reading it aloud.
// 
// In all this, the Speakable trait is still required, because it is responsible
// of translating the action and its context into a string specific for the action.
// 
// * The is_speaking and utterance_callbacks features appear to be supported
//   by Speech Dispatcher on my machine (Linux Mint 21.3, based on Ubuntu 22.04),
//   according to the Features struct that the tts crate uses to keep track
//   of which features are supported by the current backend.
//   Since the GC computers at RoboCup are also Linux-based IIRC,
//   I probably will not be testing other backends.
//   
//   My Speech Dispatcher version is as follows:
//   speech-dispatcher/jammy-updates,now 0.11.1-1ubuntu3 amd64 [installed]
//   
//   With low priority, I'll probably write an alternate, less powerful version
//   of this object that comes into play if these features are unsupported,
//   but only if this is a problem for a major backend (say, Windows's or Mac's).
//   Worst case, it'll be a simple wrapper for tts.speak,
//   without the QUEUE and without the "finished speaking a message" event,
//   and the "receive new TTS request" event will either immediately forward
//   the message to the TTS system (with interrupt=false, so if the backend has
//   an internal queue it can handle it), or outright discard the request
//   if either the HOLD or MUTE flag is on.
// 
// Refer to the tts crate's source code for all the names I used:
// https://docs.rs/crate/tts/latest/source/src/lib.rs


// Project note:
// The single object above can be created and stored in the GameController (lib.rs).
// Calls to the Speakable.speak function should no longer be made in Action.execute,
// but rather in GameController.apply, where I can also take the single object
// and pass it as an extra argument to speak.
// (Or maybe speak just produces the message
//  and apply takes care of the TTS call immediately after, I dunno.)
// So this is not quite singleton anymore, and probably a better practice
// from what I've read of Rust.
