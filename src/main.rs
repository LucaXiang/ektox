use ektox::{common::App, utils::MessageBox};

fn main() {
    match App::init() {
        Ok(app) => {
            println!("{}", app.get_version());
            app.start();
        }
        Err(error) => {
            MessageBox::error(error.to_string().as_str());
        }
    }
}
