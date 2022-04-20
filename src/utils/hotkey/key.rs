use super::SpecialKey;
#[derive(Eq)]
pub enum Key {
    AlphaNumeric(char),
    Special(SpecialKey),
}
impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::AlphaNumeric(l), Self::AlphaNumeric(r)) => l.eq(r),
            (Self::Special(l), Self::Special(r)) => l.eq(r),
            _ => false,
        }
    }
}

impl Key {
    pub fn alpha_numeric(ch: char) -> Self {
        Key::AlphaNumeric(ch)
    }

    pub fn special(special_key: SpecialKey) -> Self {
        Key::Special(special_key)
    }
}

#[cfg(test)]
mod tests {
    use super::{Key, SpecialKey};
    #[test]
    fn it_works() {
        let a = Key::AlphaNumeric('a');
        let b = Key::Special(SpecialKey::BackSpace);
        let c = Key::AlphaNumeric('a');
        assert!(a != b);
        assert!(c == a);
    }
}
