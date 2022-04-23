use self::key::Key;
use std::fmt::Error;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    HOT_KEY_MODIFIERS, MOD_ALT, MOD_CONTROL, MOD_SHIFT, MOD_WIN,
};
pub mod key;
use self::special_key::SpecialKey;
pub mod special_key;
#[derive(Eq, Debug)]
pub struct Hotkey {
    ctrl: bool,
    shift: bool,
    alt: bool,
    win: bool,
    key: Option<Key>,
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
    pub fn new(ctrl: bool, shift: bool, alt: bool, win: bool, key: Option<Key>) -> Self {
        Hotkey {
            ctrl,
            shift,
            alt,
            win,
            key,
        }
    }
    pub fn default() -> Self {
        Hotkey {
            ctrl: false,
            alt: false,
            shift: false,
            win: false,
            key: None,
        }
    }

    pub fn parse(keys: &str) -> Result<Self, Error> {
        let mut hotkey = Hotkey::default();
        let mut err = false;
        let mut part_of_keys: Vec<&str> = keys.split('+').map(|part| part.trim()).collect();
        // remove duplicate key
        // case ctrl + shift + ctrl + a
        part_of_keys.dedup();
        loop {
            // if part_of_keys is empty or just one
            // like:
            // 1: "ctrl"
            // 2: "a"
            // 3: ""
            if part_of_keys.len() < 2 {
                err = true;
                break;
            }
            for part in part_of_keys.into_iter() {
                if hotkey.parse_modifier(part) {
                    continue;
                }
                // here hotkey must be None
                // 1: "ctrl + 1 + 2"
                // 2: "ctrl + Delete + BackSpace"
                // 3: "ctrl + 1 + Delete"
                if hotkey.key == None {
                    if hotkey.parse_alpha_numeric(part) {
                        continue;
                    }
                    if hotkey.parse_special(part) {
                        continue;
                    }
                }
                err = true;
                break;
            }
            //  finaryll hotkey must contains 1 key and minimum 1 modifier
            err = err
                || !(hotkey.ctrl || hotkey.alt || hotkey.shift || hotkey.win)
                || hotkey.key == None;
            break;
        }
        if !err {
            Ok(hotkey)
        } else {
            Err(Error::default())
        }
    }

    fn parse_modifier(&mut self, str: &str) -> bool {
        match str.to_ascii_uppercase().as_str() {
            "CTRL" => {
                self.ctrl = true;
                true
            }
            "SHIFT" => {
                self.shift = true;
                true
            }
            "ALT" => {
                self.alt = true;
                true
            }
            "WIN" => {
                self.win = true;
                true
            }
            _ => false,
        }
    }
    fn parse_special(&mut self, str: &str) -> bool {
        if let Some(special_key) = SpecialKey::from_str(str) {
            self.key = Some(Key::special(special_key));
            true
        } else {
            false
        }
    }
    fn parse_alpha_numeric(&mut self, str: &str) -> bool {
        if str.len() == 1 && str.is_ascii() {
            self.key = Some(Key::AlphaNumeric(str.chars().nth(0).unwrap()));
            true
        } else {
            false
        }
    }

    pub fn get_modifiers(&self) -> HOT_KEY_MODIFIERS {
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey
        let mut modifiers: HOT_KEY_MODIFIERS = HOT_KEY_MODIFIERS::default();
        if self.alt {
            modifiers |= MOD_ALT;
        }
        if self.shift {
            modifiers |= MOD_SHIFT;
        }
        if self.ctrl {
            modifiers |= MOD_CONTROL;
        }
        if self.win {
            modifiers |= MOD_WIN;
        }
        modifiers
    }

    pub fn get_key(&self) -> u32 {
        match &self.key {
            Some(key) => key.as_u32(),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Error;

    use windows::Win32::UI::Input::KeyboardAndMouse::{MOD_ALT, MOD_CONTROL, VK_DELETE};

    use crate::utils::hotkey::special_key::SpecialKey;

    use super::{Hotkey, Key};

    #[test]
    fn it_works() {
        let new = Hotkey::new(false, false, false, false, None);
        let default = Hotkey::default();
        assert!(new == default);

        let new = Hotkey::new(true, false, false, false, Some(Key::AlphaNumeric('a')));
        let parse = Hotkey::parse("ctrl + a").unwrap();
        assert!(new == parse);

        let new = Hotkey::new(
            true,
            false,
            true,
            false,
            Some(Key::special(SpecialKey::Delete)),
        );
        let parse = Hotkey::parse("ctrl + alt + delete").unwrap();
        assert!(new == parse);
    }

    #[test]
    fn parse() {
        let parse = Hotkey::parse("ctrl + shift + f1 + f2");
        assert_eq!(parse, Err(Error::default()));

        let parse = Hotkey::parse("ctrl + f1 + f2");
        assert_eq!(parse, Err(Error::default()));

        let parse = Hotkey::parse("ctrl + shift");
        assert_eq!(parse, Err(Error::default()));

        let parse = Hotkey::parse("a + a");
        assert_eq!(parse, Err(Error::default()));

        let parse = Hotkey::parse("a + delete");
        assert_eq!(parse, Err(Error::default()));

        let parse = Hotkey::parse("backspace + delete");
        assert_eq!(parse, Err(Error::default()));

        let parse = Hotkey::parse("ababa + xxx  ss + x");
        assert_eq!(parse, Err(Error::default()));

        let parse = Hotkey::parse(" ");
        assert_eq!(parse, Err(Error::default()));
    }

    #[test]
    fn get_modifiers() {
        let hotkey = Hotkey::parse("ctrl + a").unwrap();
        assert!((hotkey.get_modifiers() & MOD_CONTROL) == MOD_CONTROL);

        let hotkey = Hotkey::parse("ctrl + alt + a").unwrap();
        assert!((hotkey.get_modifiers() & MOD_CONTROL) == MOD_CONTROL);
        assert!((hotkey.get_modifiers() & MOD_ALT) == MOD_ALT);
    }

    #[test]
    fn get_key() {
        let hotkey = Hotkey::parse("ctrl + a").unwrap();
        assert_eq!(hotkey.get_key(), 97);

        let hotkey = Hotkey::parse("ctrl + 1").unwrap();
        assert_eq!(hotkey.get_key(), 49);

        let hotkey = Hotkey::parse("ctrl + alt + delete").unwrap();
        assert_eq!(hotkey.get_key(), VK_DELETE.0 as u32);
    }
}
