use std::fmt::Display;

use self::key::Key;
use self::parse_hotkey_error::{ParseHotkeyError, ParseHotkeyErrorKind};
use self::special_key::SpecialKey;
use serde::de::Visitor;
use serde_json::value::Serializer;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    HOT_KEY_MODIFIERS, MOD_ALT, MOD_CONTROL, MOD_SHIFT, MOD_WIN,
};

pub mod key;
pub mod parse_hotkey_error;
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

    pub fn parse(source: &str) -> Result<Self, ParseHotkeyError> {
        let mut hotkey = Hotkey::default();
        let mut error = ParseHotkeyError::default();
        let mut parse_error = false;
        let part_of_keys: Vec<&str> = source.split('+').map(|part| part.trim()).collect();
        loop {
            // if part_of_keys is empty or just one
            // like:
            // 1: "ctrl"
            // 2: "a"
            // 3: ""
            if part_of_keys.len() < 2 {
                parse_error = true;
                error = ParseHotkeyError::new(source, ParseHotkeyErrorKind::KeyNotEnough);
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
                if hotkey.key != None {
                    parse_error = true;
                    error = ParseHotkeyError::new(source, ParseHotkeyErrorKind::TooManyKey);
                    break;
                }
                if hotkey.parse_alpha_numeric(part) {
                    continue;
                }
                if hotkey.parse_special(part) {
                    continue;
                }
                parse_error = true;
                error = ParseHotkeyError::new(source, ParseHotkeyErrorKind::UnexpectedKey);
                break;
            }
            //  finaryll hotkey must contains 1 key and minimum 1 modifier
            if !parse_error {
                if hotkey.key == None {
                    parse_error = true;
                    error = ParseHotkeyError::new(source, ParseHotkeyErrorKind::MissingKey);
                }
            }
            break;
        }
        if !parse_error {
            Ok(hotkey)
        } else {
            Err(error)
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

impl Display for Hotkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::default();
        if self.ctrl {
            str.push_str("Ctrl+");
        }
        if self.shift {
            str.push_str("Shift+");
        }
        if self.alt {
            str.push_str("Alt+");
        }
        if self.win {
            str.push_str("Win+");
        }
        if let Some(key) = &self.key {
            str.push_str(key.to_string().as_str());
        } else {
            str.push('?');
        }
        write!(f, "{}", str)
    }
}

impl serde::Serialize for Hotkey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Hotkey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct HotkeyVisitor;
        impl<'de> Visitor<'de> for HotkeyVisitor {
            type Value = Hotkey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expect string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match Hotkey::parse(v) {
                    Ok(hotkey) => Ok(hotkey),
                    Err(err) => Err(serde::de::Error::custom(err.to_string())),
                }
            }
        }
        deserializer.deserialize_str(HotkeyVisitor)
    }
}

#[cfg(test)]
mod tests {

    use windows::Win32::UI::Input::KeyboardAndMouse::{MOD_ALT, MOD_CONTROL, VK_DELETE};

    use crate::utils::hotkey::{
        parse_hotkey_error::{ParseHotkeyError, ParseHotkeyErrorKind},
        special_key::SpecialKey,
    };

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
        let source = "ctrl + shift + f1 + f2";
        let actual = Hotkey::parse(source);
        let expected = ParseHotkeyError::new(source, ParseHotkeyErrorKind::TooManyKey);
        assert_eq!(actual, Err(expected));

        let source = "ctrl + a + f2";
        let actual = Hotkey::parse(source);
        let expected = ParseHotkeyError::new(source, ParseHotkeyErrorKind::TooManyKey);
        assert_eq!(actual, Err(expected));

        let source = "a + a";
        let actual = Hotkey::parse(source);
        let expected = ParseHotkeyError::new(source, ParseHotkeyErrorKind::TooManyKey);
        assert_eq!(actual, Err(expected));

        let source = "ctrl + shift";
        let actual = Hotkey::parse(source);
        let expected = ParseHotkeyError::new(source, ParseHotkeyErrorKind::MissingKey);
        assert_eq!(actual, Err(expected));

        let source = "ctrl + ";
        let actual = Hotkey::parse(source);
        let expected = ParseHotkeyError::new(source, ParseHotkeyErrorKind::UnexpectedKey);
        assert_eq!(actual, Err(expected));

        let source = "";
        let actual = Hotkey::parse(source);
        let expected = ParseHotkeyError::new(source, ParseHotkeyErrorKind::KeyNotEnough);
        assert_eq!(actual, Err(expected));
    }

    #[test]
    fn get_modifiers() {
        let hotkey = Hotkey::parse("ctrl + a").unwrap();
        assert_eq!((hotkey.get_modifiers() & MOD_CONTROL), MOD_CONTROL);

        let hotkey = Hotkey::parse("ctrl + alt + a").unwrap();
        assert_eq!((hotkey.get_modifiers() & MOD_CONTROL), MOD_CONTROL);
        assert_eq!((hotkey.get_modifiers() & MOD_ALT), MOD_ALT);
    }

    #[test]
    fn get_key() {
        let hotkey = Hotkey::parse("ctrl + a").unwrap();
        assert_eq!(hotkey.get_key(), 'a' as u32);

        let hotkey = Hotkey::parse("ctrl + 1").unwrap();
        assert_eq!(hotkey.get_key(), '1' as u32);

        let hotkey = Hotkey::parse("ctrl + alt + delete").unwrap();
        assert_eq!(hotkey.get_key(), VK_DELETE.0 as u32);
    }

    #[test]
    fn to_string() {
        let hotkey = Hotkey::parse("ctrl + alt + delete").unwrap();
        assert_eq!(hotkey.to_string(), "Ctrl+Alt+Delete");
    }
}
