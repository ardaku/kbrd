//! Rust library for sending and receiving asynchronous keyboard events.

pub mod keycodes;

use keycodes::{Key, Mods as Mod};

/// Process pressing a key.
pub fn press<K: Into<u8>, F: FnMut(&Mod, u8)>(mods: &mut Mod, keycode: K, mut process: F) {
    if let Some(events) = keycodes::MAPPINGS[keycode.into() as usize] {
        for (key, modifiers) in events.iter().cloned() {
            mods.set(modifiers);
            process(mods, key as u8);
        }
    }
}

/// Process releasing a key.
pub fn release<K: Into<u8>, F: FnMut(&Mod, u8)>(mods: &mut Mod, keycode: K, mut process: F) {
    if let Some(events) = keycodes::MAPPINGS[keycode.into() as usize] {
        for (key, modifiers) in events.iter().cloned() {
            mods.clear(modifiers);
            process(mods, key as u8);
        }
    }
}

/// Modifier Keys
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
}

/// Asynchronous wait for key state changes.  Returns the new state.
pub async fn key(key: Key, mods: Option<Mods>) -> State {
    State(0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
