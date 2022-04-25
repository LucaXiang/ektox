use std::fs::File;

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
                    _ => todo!(),
                };
                Err(AppError::new(msg.to_string()))
            }
        }
    }

    pub fn get_version(&self) -> &Version {
        &self.version
    }
}
