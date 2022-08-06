//! Rust library for sending and receiving asynchronous keyboard events.

mod key;

pub use self::key::Key;
use whisk::Channel;
use pasts::prelude::*;

/// Keyboard modifiers
#[derive(Copy, Clone, Default, Debug)]
pub struct Mods(u8);

impl Mods {
    const COMPOSING: u8 = 0b1000_0000;
    const GRAPH: u8 = 0b0100_0000;
    const APP: u8 = 0b0010_0000;
    const PRG: u8 = 0b0001_0000;
    const SHIFT: u8 = 0b0000_1000;
    const EMOJI: u8 = 0b0000_0100;
    const LTB: u8 = 0b0000_0010; // Left thumb button
    const RTB: u8 = 0b0000_0001; // Right thumb button
  
    fn set(&mut self, what: u8, to: bool) {
        if to {
            self.0 |= what;
        } else {
            self.0 &= !what;
        }
    }

    fn get(&self, what: u8) -> bool {
        self.0 & what != 0
    }

    /// Create a new modifier state
    pub fn new() -> Self {
        Self::default()
    }

    /// Set composing
    pub fn set_composing(&mut self, value: bool) {
        self.set(Self::COMPOSING, value)
    }

    /// Set Graph / AltGr (linux/windows) / Option (mac) key
    pub fn set_graph(&mut self, value: bool) {
        self.set(Self::GRAPH, value)
    }

    /// Set Application / Ctrl (linux/windows) / Command (mac) key
    pub fn set_app(&mut self, value: bool) {
        self.set(Self::APP, value)
    }

    /// Set Program / Alt (linux/windows) / Control (mac) key
    pub fn set_prg(&mut self, value: bool) {
        self.set(Self::PRG, value)
    }

    /// Set Shift key
    pub fn set_shift(&mut self, value: bool) {
        self.set(Self::SHIFT, value)
    }

    /// Set Emoji key (Space key)
    pub fn set_emoji(&mut self, value: bool) {
        self.set(Self::EMOJI, value)
    }

    /// Set Left Thumb Button key
    pub fn set_ltb(&mut self, value: bool) {
        self.set(Self::LTB, value)
    }

    /// Set Right Thumb Button key
    pub fn set_rtb(&mut self, value: bool) {
        self.set(Self::RTB, value)
    }

    /// Get composing
    pub fn composing(&self) -> bool {
        self.get(Self::COMPOSING)
    }

    /// Get Graph / AltGr (linux/windows) / Option (mac) key
    pub fn graph(&self) -> bool {
        self.get(Self::GRAPH)
    }

    /// Get Application / Ctrl (linux/windows) / Command (mac) key
    pub fn app(&self) -> bool {
        self.get(Self::APP)
    }

    /// Get Program / Alt (linux/windows) / Control (mac) key
    pub fn prg(&self) -> bool {
        self.get(Self::PRG)
    }

    /// Get Shift key
    pub fn shift(&self) -> bool {
        self.get(Self::SHIFT)
    }

    /// Get Emoji key (Space key)
    pub fn emoji(&self) -> bool {
        self.get(Self::EMOJI)
    }

    /// Get Left Thumb Button key
    pub fn ltb(&self) -> bool {
        self.get(Self::LTB)
    }

    /// Get Right Thumb Button key
    pub fn rtb(&self) -> bool {
        self.get(Self::RTB)
    }
}

/// Source of keyboard events
#[derive(Clone, Debug)]
pub struct Typer(Channel<TypeInternal>);

impl Typer {
    /// Send a keyboard type event
    pub fn send(&self, typed: Type) -> impl Future<Output = ()> + Send + Unpin {
        self.0.send(typed.into())
    }
}

/// Keyboard handle
#[derive(Debug, Default)]
pub struct Keyboard {
    state: [u64; 4],
    channel: Channel<TypeInternal>,
}

impl Keyboard {
    /// Create a new keyboard
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new typer
    pub fn typer(&self) -> Typer {
        Typer(self.channel.clone())
    }
}

impl Notifier for Keyboard {
    type Event = Type;

    fn poll_next(mut self: Pin<&mut Self>, exec: &mut Exec<'_>) -> Poll<Type> {
        if let Ready(typed) = Pin::new(&mut self.channel).poll_next(exec) {
            let typed = typed.try_into().unwrap();
            // De-duplication
            match typed {
                Type::Press(key, _) => {
                    let key = key as u8;
                    let lvl = key / 64;
                    let key = key % 64;
                    if self.state[usize::from(lvl)] & (1 << key) != 0 {
                        return Pending;
                    } else {
                        self.state[usize::from(lvl)] |= 1 << key;
                    }
                }
                Type::Release(key, _) => {
                    let key = key as u8;
                    let lvl = key / 64;
                    let key = key % 64;
                    if self.state[usize::from(lvl)] & (1 << key) == 0 {
                        return Pending;
                    } else {
                        self.state[usize::from(lvl)] &= !(1 << key);
                    }
                }
                Type::Char(_) => {},
            }
            Ready(typed)
        } else {
            Pending
        }
    }
}

/// A keyboard typing event
#[repr(C, packed)]
#[derive(Debug)]
struct TypeInternal([u8; 4]);

impl TypeInternal {
    pub fn new(key: Key, state: bool, mods: Mods) -> Self {
        Self([0xFF, key as u8, state as u8, mods.0])
    }
}

impl TryFrom<TypeInternal> for Type {
    type Error = core::str::Utf8Error;

    fn try_from(other: TypeInternal) -> Result<Self, Self::Error> {
        let type_ = match other.0 {
            [0xFF, key, 0, mods] => Self::Release(Key::from(key), Mods(mods)),
            [0xFF, key, 1, mods] => Self::Press(Key::from(key), Mods(mods)),
            utf8 => Self::Char(std::str::from_utf8(&utf8)?.chars().next().unwrap()),
        };
        Ok(type_)
    }
}

/// A typing event
pub enum Type {
    Char(char),
    Press(Key, Mods),
    Release(Key, Mods),
}

impl From<Type> for TypeInternal {
    fn from(other: Type) -> Self {
        match other {
            Type::Char(other) => {
                let mut unicode = [0; 4];
                other.encode_utf8(unicode.as_mut_slice());
                Self(unicode)
            }
            Type::Press(key, mods) => {
                Self::new(key, true, mods)
            }
            Type::Release(key, mods) => {
                Self::new(key, false, mods)
            }
        }
    }
}
