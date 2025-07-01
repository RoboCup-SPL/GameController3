// TODO inclide in lib.rs

use std::collections::VecDeque;
use std::fmt::format;
use tts::*;

use std::sync::Mutex;

use std::cell::RefCell;
use std::rc::Rc;























































/// The actual TTS handler that is expected to be used
struct ActualTTSHandler {
    // storing Result here so that this struct can do all the error checking
    // and just do nothing silently if the tts initialization failed.
    // So the rest of the GameController code can go on all the same
    // regardless of the result, with minimal changes to the source.
    tts_maybe: Result<Tts, Error>,
    queue: VecDeque<String>,
    hold: bool,
    mute: bool,
}

unsafe impl Send for ActualTTSHandler {}
unsafe impl Sync for ActualTTSHandler {}

impl ActualTTSHandler {
    pub fn new() -> ActualTTSHandler {
        let tts_maybe = Tts::default();
        ActualTTSHandler {
            tts_maybe,
            queue: VecDeque::new(),
            hold: false,
            mute: false
        }
    }

    pub fn set_callback(&mut self, callback: Option<Box<dyn FnMut(UtteranceId)>>) {
        if let Ok(tts) = &mut self.tts_maybe {
            tts.on_utterance_end(callback);
        }
    }

    pub fn get_error(self) -> Option<Error> {
        match self.tts_maybe {
            Err(e) => Some(e),
            Ok(_) => None,
        }
    }

    pub fn push_tts(&mut self, message: String) {
        if !self.mute {
            self.queue.push_back(message);
            self.advance_queue();
        }
    }

    // TODO remember to use this as callback when calling the TTS crate!
    fn on_utterance_finished(&mut self) {
        println!("Finished");
        self.advance_queue();
        println!("Finished2");
    }

    // TODO keyboard events

    fn advance_queue(&mut self) {
        println!("1");
        if let Ok(tts) = &mut self.tts_maybe {
            println!("2");
            // sb else might consider adding a log instruction for the error or sth,
            // but as far as the flow is concerned, if any error happens with this
            // I'm just going to assume assume we can't speak
            let res = tts.is_speaking();
            println!("2.5");
            let free_to_speak = match res {
                Ok(is_speaking) => !is_speaking,
                Err(_) => false,  
            };
            println!("3");
            if !self.hold && !self.mute && free_to_speak {
                println!("4");
                if let Some(message) = self.queue.pop_front() {
                    println!("5");
                    // I don't care about the utterance ID
                    // and once the utterance is in the hands of the TTS backend
                    // I don't think any error is recoverable from our side,
                    // so I think this error check is safe to ignore
                    let _ = tts.speak(message, false);
                    println!("6");
                }
            }
        }
    }
}


fn factory(ctths: ClonableTTSHandler) -> Box<impl FnMut(UtteranceId)> {
    let mut another = ctths.another_one();
    let f = move |_:UtteranceId| another.on_utterance_finished();
    let boxed_f = Box::new(f);
    boxed_f
}

pub struct ClonableTTSHandler {
    actual_tts_handler: Rc<RefCell<ActualTTSHandler>>
}

unsafe impl Send for ClonableTTSHandler {}
unsafe impl Sync for ClonableTTSHandler {}

impl ClonableTTSHandler {
    pub fn new() -> ClonableTTSHandler {
        ClonableTTSHandler {
            actual_tts_handler: Rc::new(RefCell::new(ActualTTSHandler::new()))
        }
    }
    fn another_one(&self) -> ClonableTTSHandler {
        ClonableTTSHandler {
            actual_tts_handler: self.actual_tts_handler.clone()
        }
    }

    pub fn set_callback_OLD(self) {
        let another = self.another_one();
        self.actual_tts_handler.borrow_mut().set_callback(Some(factory(another)));
    }

    pub fn set_callback(&self) {
        let mut another = self.another_one();
        let f = move |_:UtteranceId| another.on_utterance_finished();
        let boxed_f = Box::new(f);
        self.actual_tts_handler.borrow_mut().set_callback(Some(boxed_f));
    }

    pub fn push_tts(&mut self, message: String) {
        self.actual_tts_handler.borrow_mut().push_tts(message);
    }

    // TODO remember to use this as callback when calling the TTS crate!
    fn on_utterance_finished(&mut self) {
        self.actual_tts_handler.borrow_mut().on_utterance_finished();
    }
}
















































// #[derive(Clone)]
// pub struct Callbacks {
//     callbacks: Vec<Rc<RefCell<dyn FnMut(&mut Shared)>>>,
// }

// impl Callbacks {
//     pub fn new() -> Self {
//         Callbacks {
//             callbacks: Vec::new(),
//         } /*@*/
//     }

//     pub fn register<F: FnMut(&mut Shared) + 'static>(&mut self, callback: F) {
//         let cell = Rc::new(RefCell::new(callback));
//         self.callbacks.push(cell); /*@*/
//     }

//     pub fn call(&mut self, val: &mut Shared) {
//         for callback in self.callbacks.iter() {
//             let mut closure = callback.borrow_mut();

//             (&mut *closure)(val);
//         }
//     }
// }

// struct Shared {
//     tts: Tts,
//     queue: VecDeque<String>,
//     hold: bool,
//     mute: bool,
// }
// impl Shared{
//     pub fn new() -> Result<Shared, Error> {
//         let tts = Tts::default()?;
//         Ok(Shared {
//             tts,
//             queue: VecDeque::new(),
//             hold: false,
//             mute: false
//         })
//     }

//     fn push_tts(&mut self, message: String) {
//         if !self.mute {
//             self.queue.push_back(message);
//             self.advance_queue();
//         }
//     }

//     fn advance_queue(&mut self) {
//             // sb else might consider adding a log instruction for the error or sth,
//         // but as far as the flow is concerned, if any error happens with this
//         // I'm just going to assume assume we can't speak
//         let free_to_speak = match self.tts.is_speaking() {
//             Ok(is_speaking) => !is_speaking,
//             Err(_) => false,  
//         };
//         if !self.hold && !self.mute && free_to_speak {
//             if let Some(message) = self.queue.pop_front() {
//                 // I don't care about the utterance ID
//                 // and once the utterance is in the hands of the TTS backend
//                 // I don't think any error is recoverable from our side,
//                 // so I think this error check is safe to ignore
//                 let _ = self.tts.speak(message, false);
//             }
//         }
//     }
// }
// struct App {
//     shared: Result<Shared, Error>,
//     events: Rc<RefCell<Callbacks>>,
// }

// impl App {
//     fn new() -> App {
//         let mut new_app = App {
//             shared: Shared::new(),
//             events: Rc::new(RefCell::new(Callbacks::new())),
//         };
//         // TODO can just add this in the initialization of Callbacks, maybe?
//         new_app.insert(|val| {
//             val.advance_queue();
//         });
//         // if let Ok(good_shared) = new_app.shared {
//         //     let f = |_:UtteranceId| new_app.call_events();
//         //     let boxed_f = Box::new(f);
//         //     good_shared.tts.on_utterance_end(Some(boxed_f));
//         // }
//         new_app
//     }

//     fn get_error(self) -> Option<Error> {
//         match self.shared {
//             Err(e) => Some(e),
//             Ok(_) => None,
//         }
//     }

//     fn insert<F: FnMut(&mut Shared) + 'static>(&mut self, event: F) {
//         self.events.borrow_mut().register(event);
//     }

//     pub fn push_tts(&mut self, message: String) {
//         if let Ok(good_shared) = &mut self.shared {
//             good_shared.push_tts(message);
//         }
//     }

//     fn call_events(&mut self) {
//         if let Ok(good_shared) = &mut self.shared {
//             self.events.borrow_mut().call(good_shared);
//         }
//     }
// }

// fn main() {
//     let mut app = App::new();

//     app.insert(|val| {
//         val.advance_queue();
//     });

//     app.call_events();
// }











































// pub struct TTSHandlerNoTts<'a> {
//     queue: VecDeque<String>,
//     hold: bool,
//     mute: bool,
// }

// impl<'a> TTSHandlerNoTts<'a> {
//     pub fn new() -> TTSHandlerNoTts<'a> {
//         TTSHandlerNoTts {
//             queue: VecDeque::new(),
//             hold: false,
//             mute: false
//         }
//     }

//     pub fn push_tts(&mut self, message: String) {
//         if !self.mute {
//             self.queue.push_back(message);
//             self.advance_queue();
//         }
//     }

//     // TODO remember to use this as callback when calling the TTS crate!
//     fn on_utterance_finished(&mut self) {
//         self.advance_queue();
//     }

//     // TODO keyboard events

//     fn advance_queue<'b:'static>(&mut self) {
//         if !self.hold && !self.mute {
//             if let Some(message) = self.queue.pop_front() {
//                 // THIS FN IS DONE W/ SELF NOW
//                 if let Ok(mut tts) = Tts::default() {
//                     // sb else might consider adding a log instruction for the error or sth,
//                     // but as far as the flow is concerned, if any error happens with this
//                     // I'm just going to assume assume we can't speak
//                     let free_to_speak = match tts.is_speaking() {
//                         Ok(is_speaking) => !is_speaking,
//                         Err(_) => false,  
//                     };
//                     if free_to_speak {
//                         let boxed_f: Box<dyn FnMut(UtteranceId) + 'a> = Box::new(|_:UtteranceId| TTSHandlerNoTts::on_utterance_finished(self));
//                         tts.on_utterance_end(Some(boxed_f));
//                         // I don't care about the utterance ID
//                         // and once the utterance is in the hands of the TTS backend
//                         // I don't think any error is recoverable from our side,
//                         // so I think this error check is safe to ignore
//                         let _ = tts.speak(message, false);
//                     }
//                 }
//             }
//         }
//     }
// }

























pub struct TTSHandlerUnderMutex {
    tts: Tts,
    queue: VecDeque<String>,
    hold: bool,
    mute: bool,
}

pub struct TTSHandlerPublic {
    // storing Result here so that this struct can do all the error checking
    // and just do nothing silently if the tts initialization failed.
    // So the rest of the GameController code can go on all the same
    // regardless of the result, with minimal changes to the source.
    protected_stuff: Result<Mutex<TTSHandlerUnderMutex>, Error>
}


impl TTSHandlerUnderMutex {
    pub fn new() -> Result<TTSHandlerUnderMutex, Error> {
        Ok(TTSHandlerUnderMutex {
            tts: Tts::default()?,
            queue: VecDeque::new(),
            hold: false,
            mute: false
        })
    }

    pub fn set_callback(&mut self, callback: Option<Box<dyn FnMut(UtteranceId)>>) {
        self.tts.on_utterance_end(callback);
    }

    pub fn push_tts(&mut self, message: String) {
        if !self.mute {
            self.queue.push_back(message);
            self.advance_queue();
        }
    }

    fn on_utterance_finished(&mut self) {
        self.advance_queue();
    }

    // TODO keyboard events

    fn advance_queue(&mut self) {
        // sb else might consider adding a log instruction for the error or sth,
        // but as far as the flow is concerned, if any error happens with this
        // I'm just going to assume assume we can't speak
        let free_to_speak = match self.tts.is_speaking() {
            Ok(is_speaking) => !is_speaking,
            Err(_) => false,  
        };
        if !self.hold && !self.mute && free_to_speak {
            if let Some(message) = self.queue.pop_front() {
                // I don't care about the utterance ID
                // and once the utterance is in the hands of the TTS backend
                // I don't think any error is recoverable from our side,
                // so I think this error check is safe to ignore
                let _ = self.tts.speak(message, false);
            }
        }
    }
}

impl TTSHandlerPublic {
    pub fn new() -> TTSHandlerPublic {
        match TTSHandlerUnderMutex::new() {
            Ok(ttsh) => TTSHandlerPublic {
                protected_stuff: Ok(Mutex::new(ttsh))
            },
            Err(e) => TTSHandlerPublic {
                protected_stuff: Err(e)
            }
        }
    }

    // pub fn set_callback(&mut self) {
    //     // let f = |_:UtteranceId| TTSHandlerPublic::on_utterance_finished(self);
    //     let boxed_f: Box<dyn FnMut(UtteranceId)> = Box::new(Self::on_utterance_finished);
    //     if let Ok(mutex) = &mut self.protected_stuff {
    //         if let Ok(tts_handler) = &mut mutex.lock() {
    //             tts_handler.set_callback(Some(boxed_f));
    //         }
    //     }
    // }

    pub fn get_error(self) -> Option<Error> {
        match self.protected_stuff {
            Err(e) => Some(e),
            Ok(_) => None,
        }
    }

    fn on_utterance_finished(&mut self, _: UtteranceId) {
        if let Ok(mutex) = &mut self.protected_stuff {
            if let Ok(tts_handler) = &mut mutex.lock() {
                tts_handler.on_utterance_finished();
            }
        }
    }

    pub fn push_tts(&mut self, message: String) {
        if let Ok(mutex) = &mut self.protected_stuff {
            if let Ok(tts_handler) = &mut mutex.lock() {
                tts_handler.push_tts(message);
            }
        }
    }
}

















// mi sono rotto, famo senza struct e tutto unsafe

static mut TTS_MAYBE: Result<Tts, Error> = Err(Error::NoneError);
static mut QUEUE: VecDeque<String> = VecDeque::new();
static mut HOLD: bool = false;
static mut MUTE: bool = false;

pub unsafe fn ttshandle_init() {
    TTS_MAYBE = Tts::default();
    QUEUE = VecDeque::new();
    let boxed_f = Box::new(ttshandle_on_utterance_finished);
    if let Ok(tts) = TTS_MAYBE.as_ref() {
        tts.on_utterance_end(Some(boxed_f));
    }
}

pub unsafe fn ttshandle_push_tts(message: String) {
    if !MUTE {
        QUEUE.push_back(message);
        ttshandle_advance_queue();
    }
}

fn ttshandle_on_utterance_finished(_: UtteranceId) {
    unsafe {
        ttshandle_advance_queue();
    }
}

// TODO keyboard events

unsafe fn ttshandle_advance_queue() {
    println!("advancing");
    if let Ok(tts) = TTS_MAYBE.as_mut() {
        println!("past tts");
        // sb else might consider adding a log instruction for the error or sth,
        // but as far as the flow is concerned, if any error happens with this
        // I'm just going to assume assume we can't speak
        let free_to_speak = match tts.is_speaking() {
            Ok(is_speaking) => !is_speaking,
            Err(_) => false,  
        };
        println!("past free_to_speak");
        if !HOLD && !MUTE && free_to_speak {
            println!("past if");
            if let Some(message) = QUEUE.pop_front() {
                println!("found message, speaking...");
                // I don't care about the utterance ID
                // and once the utterance is in the hands of the TTS backend
                // I don't think any error is recoverable from our side,
                // so I think this error check is safe to ignore
                let _ = tts.speak(message, false);
            }
        }
    }
}




// ... o forse mutex

use lazy_static::*;

lazy_static! {
    static ref TTS_MAYBE_2: Mutex<Result<Tts, Error>> = {
        let m: Result<Tts, Error> = Tts::default();
        Mutex::new(m)
    };
    static ref QUEUE_2: Mutex<VecDeque<String>> = {
        let m: VecDeque<String> = VecDeque::new();
        Mutex::new(m)
    };
}

pub fn ttshandle2_init() {
    let boxed_f = Box::new(ttshandle_on_utterance_finished);
    let tts_maybe = TTS_MAYBE_2.lock().unwrap();
    if let Ok(tts) = tts_maybe.as_ref() {
        tts.on_utterance_end(Some(boxed_f));
    }
}

pub fn ttshandle2_push_tts(message: String) {
    // TODO mute flag
    let mut queue = QUEUE_2.lock().unwrap();
    queue.push_back(message);
    ttshandle2_advance_queue();
}

fn ttshandle2_on_utterance_finished(_: UtteranceId) {
    ttshandle2_advance_queue();
}

// TODO keyboard events

fn ttshandle2_advance_queue() {
    println!("advancing");
    let tts_maybe = TTS_MAYBE_2.lock().unwrap();
    let mut queue = QUEUE_2.lock().unwrap();
    // if let Ok(tts) = tts_maybe.as_mut() {
    //     println!("past tts");
    //     // sb else might consider adding a log instruction for the error or sth,
    //     // but as far as the flow is concerned, if any error happens with this
    //     // I'm just going to assume assume we can't speak
    //     let free_to_speak = match tts.is_speaking() {
    //         Ok(is_speaking) => !is_speaking,
    //         Err(_) => false,  
    //     };
    //     println!("past free_to_speak");
    //     // TODO hold and mute flags
    //     if free_to_speak {
    //         println!("past if");
    //         if let Some(message) = queue.pop_front() {
    //             println!("found message, speaking...");
    //             // I don't care about the utterance ID
    //             // and once the utterance is in the hands of the TTS backend
    //             // I don't think any error is recoverable from our side,
    //             // so I think this error check is safe to ignore
    //             let _ = tts.speak(message, false);
    //         }
    //     }
    // }
}













/// The actual TTS handler that is expected to be used
pub struct FullTTSHandler {
    // storing Result here so that this struct can do all the error checking
    // and just do nothing silently if the tts initialization failed.
    // So the rest of the GameController code can go on all the same
    // regardless of the result, with minimal changes to the source.
    tts_maybe: Result<Tts, Error>,
    queue: VecDeque<String>,
    hold: bool,
    mute: bool,
}

impl FullTTSHandler {
    pub fn new() -> FullTTSHandler {
        let tts_maybe = Tts::default();
        FullTTSHandler {
            tts_maybe,
            queue: VecDeque::new(),
            hold: false,
            mute: false,
        }
    }

    pub fn callback(&mut self, _:UtteranceId) {
        self.on_utterance_finished();
    }

    // pub fn set_callback(&mut self) {
    //     let f = |_:UtteranceId| FullTTSHandler::on_utterance_finished(self);
    //     let boxed_f = Box::new(f);
    //     if let Ok(tts) = &mut self.tts_maybe {
    //         tts.on_utterance_end(Some(boxed_f));
    //     }
    // }

    pub fn get_error(self) -> Option<Error> {
        match self.tts_maybe {
            Err(e) => Some(e),
            Ok(_) => None,
        }
    }

    pub fn push_tts(&mut self, message: String) {
        if !self.mute {
            self.queue.push_back(message);
            self.advance_queue();
        }
    }

    // TODO remember to use this as callback when calling the TTS crate!
    fn on_utterance_finished(&mut self) {
        self.advance_queue();
    }

    // TODO keyboard events

    fn advance_queue(&mut self) {
        if let Ok(tts) = &mut self.tts_maybe {
            // sb else might consider adding a log instruction for the error or sth,
            // but as far as the flow is concerned, if any error happens with this
            // I'm just going to assume assume we can't speak
            let free_to_speak = match tts.is_speaking() {
                Ok(is_speaking) => !is_speaking,
                Err(_) => false,  
            };
            if !self.hold && !self.mute && free_to_speak {
                if let Some(message) = self.queue.pop_front() {
                    // I don't care about the utterance ID
                    // and once the utterance is in the hands of the TTS backend
                    // I don't think any error is recoverable from our side,
                    // so I think this error check is safe to ignore
                    let _ = tts.speak(message, false);
                }
            }
        }
    }
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
