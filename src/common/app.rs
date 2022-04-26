use std::fs::File;

use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{GetMessageW, MSG, WM_HOTKEY, WM_QUIT},
};

use super::{AppError, Config, Version};

pub struct App {
    version: Version,
    config: Config,
}
impl App {
    pub fn init() -> Result<Self, AppError> {
        let version = Version::from_cargo_package();
        let config = Self::load_configure()?;

        return Ok(App { version, config });
    }

    pub fn start(&self) {
        self.handle_window_event()
    }

    fn handle_window_event(&self) {
        let mut msg: MSG = MSG::default();
        unsafe {
            while GetMessageW(&mut msg, HWND::default(), 0, 0).into() {
                match msg.message {
                    WM_HOTKEY => {
                        let id: usize = msg.wParam.0;
                        self.process(id);
                    }
                    WM_QUIT => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
    fn process(&self, id: usize) {
        todo!()
    }

    fn load_configure() -> Result<Config, AppError> {
        match File::open("./config.json") {
            Ok(file) => match serde_json::from_reader::<File, Config>(file) {
                Ok(config) => Ok(config),
                Err(error) => Err(AppError::new(error.to_string())),
            },
            Err(io_error) => {
                let msg = match io_error.kind() {
                    std::io::ErrorKind::NotFound => "Configure File Not Found",
                    std::io::ErrorKind::PermissionDenied => " Open Configure File Permision Denied",
                    _ => "Unexpected error",
                };
                Err(AppError::new(msg.to_string()))
            }
        }
    }

    pub fn get_version(&self) -> &Version {
        &self.version
    }
}
