use super::Version;

pub struct App {
    version: Version,
}
impl App {
    pub fn new() -> Self {
        App {
            version: Version::from_cargo_package(),
        }
    }
    pub fn get_version(&self) -> &Version {
        &self.version
    }
}
