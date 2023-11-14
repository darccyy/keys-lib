use crate::{Key, Modifiers};
use ggez::input::keyboard::{KeyInput, KeyMods};

impl TryFrom<KeyInput> for Key {
    type Error = ();
    fn try_from(input: KeyInput) -> Result<Self, Self::Error> {
        let name = input.keycode.ok_or(())?.try_into()?;
        let modifiers = input.mods.try_into()?;
        Ok(Key { name, modifiers })
    }
}

impl TryFrom<KeyMods> for Modifiers {
    type Error = ();
    fn try_from(mods: KeyMods) -> Result<Self, Self::Error> {
        let mut modifiers = Modifiers::default();

        if mods.contains(KeyMods::SHIFT) {
            modifiers.shift = true;
        }
        if mods.contains(KeyMods::CTRL) {
            modifiers.control = true;
        }
        if mods.contains(KeyMods::ALT) {
            modifiers.alt = true;
        }
        if mods.contains(KeyMods::LOGO) {
            return Err(());
        }

        Ok(modifiers)
    }
}
