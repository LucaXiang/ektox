use super::SpecialKey;
#[derive(Eq, Debug)]
pub enum Key {
    AlphaNumeric(char),
    Special(SpecialKey),
}
impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::AlphaNumeric(l), Self::AlphaNumeric(r)) => {
                l.to_ascii_uppercase().eq(&r.to_ascii_uppercase())
            }
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

    pub fn as_u32(&self) -> u32 {
        match self {
            Key::AlphaNumeric(c) => *c as u32,
            Key::Special(k) => k.as_u32(),
        }
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
        assert!(a.as_u32() == 97);
        assert!(b.as_u32() == 8)
    }
}
