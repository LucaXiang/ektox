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
            shift: false,
            alt: false,
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
        for key in part_of_keys {
            let str = key.to_uppercase();
            let str = str.as_str();
            if match str {
                "CTRL" => {
                    hotkey.ctrl = true;
                    true
                }
                "ALT" => {
                    hotkey.alt = true;
                    true
                }
                "SHIFT" => {
                    hotkey.shift = true;
                    true
                }
                "WIN" => {
                    hotkey.win = true;
                    true
                }
                _ => false,
            } {
                continue;
            }
            // at here key must be None
            // ctrl + f1 + f2  (Special + Special)
            // ctrl + a + b    (AlphaNumeric + AlphaNumeric)
            // ctrl + f1 + a   (Special + AlphaNumeric)
            if hotkey.key == None {
                if let Some(special_key) = SpecialKey::from_str(str) {
                    hotkey.key = Some(Key::Special(special_key));
                    continue;
                }
                // at here key must be alpha numeric
                if str.len() == 1 && str.is_ascii() {
                    hotkey.key = Some(Key::AlphaNumeric(str.chars().nth(0).unwrap()));
                    continue;
                }
            }
            // parse error
            err = true;
            break;
        }
        if !err {
            Ok(hotkey)
        } else {
            Err(Error::default())
        }
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

    use super::{Hotkey, Key};

    #[test]
    fn it_works() {
        let new = Hotkey::new(false, false, false, false, None);
        let default = Hotkey::default();
        assert!(new == default);
    }

    #[test]
    fn parse_1() {
        let new = Hotkey::new(true, false, false, false, Some(Key::AlphaNumeric('a')));
        let parse = Hotkey::parse("ctrl + a").unwrap();
        println!("{:?}", new);
        println!("{:?}", parse);
        assert!(new == parse);
    }
    #[test]
    fn parse_2() {
        let new = Hotkey::new(true, false, false, false, Some(Key::AlphaNumeric('a')));
        let parse = Hotkey::parse("ctrl + ctrl + a").unwrap();
        println!("{:?}", new);
        println!("{:?}", parse);
        assert!(new == parse);
    }

    #[test]
    fn parse_3() {
        let parse = Hotkey::parse("ctrl + ctrl + f1 + f2");
        assert_eq!(parse, Err(Error::default()));
    }
}
