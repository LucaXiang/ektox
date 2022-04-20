use std::string::ParseError;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    HOT_KEY_MODIFIERS, MOD_ALT, MOD_CONTROL, MOD_SHIFT, MOD_WIN,
};

use self::key::Key;
pub mod key;
use self::special_key::SpecialKey;
pub mod special_key;
#[derive(Eq)]
struct Hotkey {
    ctrl: bool,
    shift: bool,
    alt: bool,
    win: bool,
    key: Key,
}
impl PartialEq for Hotkey {
    fn eq(&self, other: &Self) -> bool {
        self.ctrl == other.ctrl
            && self.shift == other.shift
            && self.alt == other.alt
            && self.win == other.win
            && self.key == other.key
    }
}
impl Hotkey {
    pub fn new(ctrl: bool, shift: bool, alt: bool, win: bool, key: Key) -> Self {
        Hotkey {
            ctrl,
            shift,
            alt,
            win,
            key,
        }
    }
    pub fn default() -> Self {
        Hotkey::new(false, false, false, false, Key::alpha_numeric('0'))
    }

    pub fn parse(s: &String) -> Result<Self, ParseError> {
        let mut copy = s.clone();
        Ok(Hotkey::default())
    }

    pub fn get_modifiers(&self) -> HOT_KEY_MODIFIERS {
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey
        let mut modifiers: HOT_KEY_MODIFIERS = HOT_KEY_MODIFIERS::default();
        if self.alt {
            modifiers |= MOD_ALT;
        }
        if self.ctrl {
            modifiers |= MOD_CONTROL;
        }
        if self.shift {
            modifiers |= MOD_SHIFT;
        }
        if self.win {
            modifiers |= MOD_WIN;
        }
        modifiers
    }
}

#[cfg(test)]
mod tests {
    use super::{Hotkey, Key};

    #[test]
    fn it_works() {
        let new = Hotkey::new(false, false, false, false, Key::AlphaNumeric('0'));
        let default = Hotkey::default();
        assert!(new == default);
    }
}
