#[derive(Eq)]
struct Hotkey {
    ctrl: bool,
    shift: bool,
    alt: bool,
    win: bool,
    key: isize,
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
    pub fn new(ctrl: bool, shift: bool, alt: bool, win: bool, key: isize) -> Self {
        Hotkey {
            ctrl,
            shift,
            alt,
            win,
            key,
        }
    }
    pub fn default() -> Self {
        Hotkey::new(false, false, false, false, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::Hotkey;

    #[test]
    fn it_works() {
        let new = Hotkey::new(false, false, false, false, 0);
        let default = Hotkey::default();
        assert!(new == default);
    }
}
