pub mod keycodes;

use keycodes::Mods;

/// Process pressing a key.
pub fn press<K: Into<u8>, F: FnMut(&Mods, u8)>(mods: &mut Mods, keycode: K, mut process: F) {
    if let Some(events) = keycodes::MAPPINGS[keycode.into() as usize] {
        for (key, modifiers) in events.iter().cloned() {
            mods.set(modifiers);
            process(mods, key.into());
        }
    }
}
/// Process releasing a key.
pub fn release<K: Into<u8>, F: FnMut(&Mods, u8)>(mods: &mut Mods, keycode: K, mut process: F) {
    if let Some(events) = keycodes::MAPPINGS[keycode.into() as usize] {
        for (key, modifiers) in events.iter().cloned() {
            mods.clear(modifiers);
            process(mods, key.into());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
