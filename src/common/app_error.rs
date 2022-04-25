use std::fmt::Display;

pub struct AppError {
    msg: String,
}

impl AppError {
    pub fn new(msg: String) -> Self {
        AppError { msg }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
