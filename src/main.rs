#![windows_subsystem = "windows"]
use ektox::{common::App, utils::MessageBox};

fn main() {
    match App::init() {
        Ok(app) => {
            app.start();
        }
        Err(error) => {
            MessageBox::error(error.to_string().as_str());
        }
    }
}
