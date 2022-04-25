use std::fmt::Display;

#[derive(Debug, Eq)]
pub enum ParseHotkeyErrorKind {
    KeyNotEnough,
    TooManyKey,
    MissingKey,
    UnexpectedKey,
    Unknow,
}

impl PartialEq for ParseHotkeyErrorKind {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

#[derive(Debug, Eq)]
pub struct ParseHotkeyError {
    source: String,
    kind: ParseHotkeyErrorKind,
}

impl ParseHotkeyError {
    pub fn default() -> Self {
        ParseHotkeyError {
            source: "unexpected error".to_string(),
            kind: ParseHotkeyErrorKind::Unknow,
        }
    }

    pub fn new(source: &str, kind: ParseHotkeyErrorKind) -> Self {
        ParseHotkeyError {
            source: source.to_string(),
            kind,
        }
    }
}
impl Display for ParseHotkeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = format!("{:#?}", self.kind);
        write!(
            f,
            "ParseHotkeyError: '{}' from source '{}'",
            message, self.source
        )
    }
}

impl PartialEq for ParseHotkeyError {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && self.kind == other.kind
    }
}
