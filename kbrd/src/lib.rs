//! Rust library for sending and receiving asynchronous keyboard events.

pub mod keycodes;

use std::{future::Future, pin::Pin, task::{Poll, Context}};

use keycodes::{Key, Mods as Mod};
use flume::{Sender, Receiver, bounded};

lazy_static::lazy_static! {
    static ref KEYBOARD: (Sender<(Key, State)>, Receiver<(Key, State)>) = bounded(8);
}

/// Process pressing a key.
pub fn press<K: Into<u8>>(sender: &Sender<(Key, State)>, mods: &mut Mod, keycode: K) {
    if let Some(events) = keycodes::MAPPINGS[keycode.into() as usize] {
        for (key, modifiers) in events.iter().cloned() {
            mods.set(modifiers);
            if sender.try_send((key, State(mods.0 | PRESS))).is_err() {
                // FIXME: Use devout's say! macro.
                eprintln!("[CALA] Too slow!!! Dropping input events.");
            }
        }
    }
}

/// Process releasing a key.
pub fn release<K: Into<u8>>(sender: &Sender<(Key, State)>, mods: &mut Mod, keycode: K) {
    if let Some(events) = keycodes::MAPPINGS[keycode.into() as usize] {
        for (key, modifiers) in events.iter().cloned() {
            mods.clear(modifiers);
            if sender.try_send((key, State(mods.0))).is_err() {
                // FIXME: Use devout's say! macro.
                eprintln!("[CALA] Too slow!!! Dropping input events.");
            }
        }
    }
}

/// Modifier Keys
#[repr(u8)]
pub enum Mods {
    /// No modifiers
    None = 0b0000_0000,
    /// "Shift"
    Shift = 0b1000_0000,
    /// "Control + Shift" (Windows, Linux, Rynvei), "Left Alt" (Windows, Linux,
    /// Rynvei), "Control" (Mac), "Cmd + Shift" (Mac)
    Alt = 0b1100_0000,
    /// "Left Cmd" (Mac), "Ctrl" (Windows, Linux, Rynvei)
    Ctrl = 0b0100_0000,
    /// "Left Option" (Mac, Rynvei), "AltGr" (Windows) or "Right Alt" (Linux)
    Opt = 0b0010_0000,
    /// "Left Option + Shift" (Mac, Rynvei), "AltGr + Shift" (Windows, Linux)
    ShiftOpt = 0b1010_0000,
    /// Also "Left Thumb + Right Thumb"
    CtrlOpt = 0b0110_000,
}

/// Repeat
const REPEAT: u8 = 0b0000_0010;
/// Pressed
const PRESS: u8 = 0b0000_0001;
/// "Right Option" (Mac), "Insert" (Windows, Linux)
const COMPOSE: u8 = 0b0001_0000;
/// "CapsLock" (Windows, Linux, Mac)
const CLIPBOARD: u8 = 0b0000_1000;
/// "Right Control" (Windows, Linux), "Right Cmd" (Mac)
const EMOJI: u8 = 0b0000_0100;

/// State of a key.
#[repr(transparent)]
pub struct State(u8);

impl State {
    /// Check if key state got changed to PRESSED (true) or RELEASED (false).
    #[inline(always)]
    pub fn pressed(&self) -> bool {
        self.0 & (PRESS | REPEAT) == PRESS
    }

    /// Textual input repeated key.
    #[inline(always)]
    pub fn repeated(&self) -> bool {
        self.0 & REPEAT == REPEAT
    }

    /// Key either repeated or pressed.
    #[inline(always)]
    pub fn inputted(&self) -> bool {
        self.0 & (PRESS | REPEAT) != 0
    }
    
    /// Check if key pressed with modifiers.
    #[inline(always)]
    pub fn pressed_with(&self, mods: Mods) -> bool {
        (self.0 & (0b1110_0000 | PRESS)) == (mods as u8 | PRESS)
    }
}

struct Keyboard;

impl Future for Keyboard {
    type Output = (Key, State);

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

/// Asynchronous wait watching for key state changes.  Returns which key and the
/// new state.
pub async fn key() -> (Key, State) {
    Keyboard.await
}
