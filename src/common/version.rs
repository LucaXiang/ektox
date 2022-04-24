use std::fmt;

#[derive(Eq, Ord, Clone, Debug)]
pub struct Version {
    major: i32,
    minor: i32,
    patch: i32,
    version: String,
}

impl Version {
    #[allow(unused)]
    pub fn new(version: &'static str) -> Self {
        let mut version_parts = version.split('.');
        let major = version_parts.next().unwrap().parse().unwrap();
        let minor = version_parts.next().unwrap().parse().unwrap();
        let patch = version_parts.next().unwrap().parse().unwrap();
        Version {
            major,
            minor,
            patch,
            version: format!("{}.{}.{}", major, minor, patch),
        }
    }
    #[allow(unused)]
    pub fn from_cargo_package() -> Self {
        Version {
            major: env!("CARGO_PKG_VERSION_MAJOR").parse::<i32>().unwrap(),
            minor: env!("CARGO_PKG_VERSION_MINOR").parse::<i32>().unwrap(),
            patch: env!("CARGO_PKG_VERSION_PATCH").parse::<i32>().unwrap(),
            version: String::from(env!("CARGO_PKG_VERSION")),
        }
    }
    #[allow(unused)]
    pub fn get_major(&self) -> i32 {
        self.major
    }
    #[allow(unused)]
    pub fn get_minor(&self) -> i32 {
        self.minor
    }
    #[allow(unused)]
    pub fn get_patch(&self) -> i32 {
        self.patch
    }
    #[allow(unused)]
    pub fn get_version(&self) -> &String {
        &self.version
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<std::cmp::Ordering> {
        let mut result = std::cmp::Ordering::Equal;
        loop {
            if self.major != other.major {
                result = self.major.cmp(&other.major);
                break;
            }
            if self.minor != other.minor {
                result = self.minor.cmp(&other.minor);
                break;
            }
            if self.patch != other.patch {
                result = self.patch.cmp(&other.patch);
                break;
            }
            break;
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::Version;

    #[test]
    fn it_works() {
        let str = "1.0.0";
        let version = Version::new(str);
        assert_eq!(version.get_major(), 1);
        assert_eq!(version.get_minor(), 0);
        assert_eq!(version.get_patch(), 0);
        assert_eq!(version.get_version(), str);
    }
    #[test]
    fn from_cargo_package() {
        let env_version = Version::from_cargo_package();
        let test_version = Version::new(env!("CARGO_PKG_VERSION"));
        assert_eq!(env_version, test_version);
    }

    #[test]
    fn compare() {
        let old_version = Version::new("1.0.0");
        let new_version = Version::new("1.0.1");
        assert!(old_version < new_version);

        let old_version = Version::new("1.0.5");
        let new_version = Version::new("2.0.1");
        assert!(new_version > old_version);

        let old_version = Version::new("1.1.1");
        let new_version = Version::new("1.1.1");
        assert!(new_version == old_version);
    }
}
