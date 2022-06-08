#![allow(unused)]

use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    fmt::{Debug, Display},
    str::FromStr,
    time::SystemTime,
};

use serde::{de::EnumAccess, Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

impl SemVer {
    pub fn new(major: u16, minor: u16, patch: u16) -> SemVer {
        SemVer {
            major,
            minor,
            patch,
        }
    }

    pub fn new_short(major: u16) -> SemVer {
        Self::new(major, 0, 0)
    }

    pub fn info(&self) {
        println!(
            "hi, I'm a semver: {}.{}.{}",
            self.major, self.minor, self.patch
        );
    }

    pub fn patch(&mut self) -> &mut u16 {
        &mut self.patch
    }

    fn demo(v: Vec<u16>) -> [u16; 3] {
        let r = v.try_into();
        r.unwrap()
    }
}

impl Default for SemVer {
    fn default() -> Self {
        Self::new_short(1)
    }
}

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

fn parse_and_convert_to_option(s: &str) -> Option<u16> {
    s.parse().ok()
}

impl FromStr for SemVer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(".").map(|part| part.parse());
        let mut next_part = |x| {
            let maybe_num = it.next().ok_or_else(|| format!("{} is missing", x))?;
            maybe_num.map_err(|e| format!("{} is invalid", x))
        };
        let semver = Ok(SemVer {
            major: next_part("major")?,
            minor: next_part("minor")?,
            patch: next_part("patch")?,
        });

        match it.next() {
            Some(_) => Err("invalid trailing input".to_string()),
            None => semver,
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Program {
    name: String,
    release_history: Vec<Release>,
}

impl Program {
    pub fn new<N: AsRef<str>>(name: N) -> Self {
        let name_ref = name.as_ref();
        let name = name_ref.to_string();
        Program {
            name,
            release_history: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Library {
    name: String,
    release_history: Vec<Release>,
}

impl Library {
    pub fn new<N: AsRef<str>>(name: N) -> Self {
        let name_ref = name.as_ref();
        let name = name_ref.to_string();
        Library {
            name,
            release_history: vec![],
        }
    }
}

impl PartialEq for Library {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialEq for Program {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Library {}
impl Eq for Program {}

pub trait CrateFuncs {
    fn name(&self) -> &str;
    fn latest_release(&self) -> Option<&Release>;
    fn add_release(&mut self, release: Release);
}

impl CrateFuncs for Program {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn latest_release(&self) -> Option<&Release> {
        self.release_history.last()
    }

    fn add_release(&mut self, release: Release) -> () {
        self.release_history.push(release);
    }
}

impl CrateFuncs for Library {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn latest_release(&self) -> Option<&Release> {
        self.release_history.last()
    }

    fn add_release(&mut self, release: Release) -> () {
        self.release_history.push(release);
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub enum Crate {
    Library(Library),
    Program(Program),
}

// forwarding implementations can get tedious fast -
// the enum_dispatch crate can help here
impl CrateFuncs for Crate {
    fn name(&self) -> &str {
        match self {
            Crate::Library(lib) => lib.name(),
            Crate::Program(bin) => bin.name(),
        }
    }

    fn latest_release(&self) -> Option<&Release> {
        match self {
            Crate::Library(lib) => lib.latest_release(),
            Crate::Program(bin) => bin.latest_release(),
        }
    }

    fn add_release(&mut self, version: Release) -> () {
        match self {
            Crate::Library(lib) => lib.add_release(version),
            Crate::Program(bin) => bin.add_release(version),
        }
    }
}

impl TryFrom<&Crate> for String {
    type Error = serde_json::Error;

    fn try_from(value: &Crate) -> Result<Self, Self::Error> {
        serde_json::to_string(value)
    }
}

impl TryFrom<Crate> for String {
    type Error = serde_json::Error;

    fn try_from(value: Crate) -> Result<Self, Self::Error> {
        serde_json::to_string(&value)
    }
}

impl FromStr for Crate {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
pub struct Release {
    pub version: SemVer,
    pub timestamp: SystemTime,
}

impl Release {
    pub fn new(version: SemVer) -> Self {
        Self {
            version,
            timestamp: SystemTime::now(),
        }
    }

    pub fn new_with_timestamp(version: SemVer, when: SystemTime) -> Self {
        Self {
            version,
            timestamp: when,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EnumRepository {
    pub crates: HashMap<String, Crate>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    NotFound,
    InvalidVersion,
}

impl EnumRepository {
    pub fn new() -> Self {
        EnumRepository {
            crates: HashMap::default(),
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.crates.contains_key(name)
    }

    pub fn insert(&mut self, crt: Crate) -> bool {
        self.crates.insert(crt.name().to_owned(), crt).is_none()
    }

    pub fn get(&self, name: &str) -> Result<&Crate, Error> {
        self.crates.get(name).ok_or(Error::NotFound)
    }

    pub fn add_release<N: AsRef<str>>(&mut self, name: N, version: SemVer) -> Result<(), Error> {
        let crt = self.crate_mut(name)?;
        if let Some(latest) = crt.latest_release() {
            // only allow newer version numbers
            if version <= latest.version {
                return Err(Error::InvalidVersion);
            }
        }
        crt.add_release(Release::new(version));
        Ok(())
    }

    fn crate_mut<N: AsRef<str>>(&mut self, name: N) -> Result<&mut Crate, Error> {
        self.crates.get_mut(name.as_ref()).ok_or(Error::NotFound)
    }
}

#[cfg(test)]
mod tests {

    use std::error::Error;

    use super::*;

    #[test]
    fn release() {
        let mut lib = Library::new("hello_lib");
        assert_eq!(lib.latest_release(), None);

        let now = SystemTime::now();
        lib.add_release(Release::new_with_timestamp(SemVer::new_short(1), now));
        assert_eq!(
            lib.latest_release(),
            Some(&Release::new_with_timestamp(SemVer::new_short(1), now))
        );
    }

    #[test]
    fn test_serialize() -> Result<(), Box<dyn Error>> {
        let p = Crate::Program(Program::new("hello_bin"));
        let s: String = p.try_into()?;
        Ok(())
    }

    #[test]
    fn from_str() {
        assert_eq!(
            SemVer::from_str("x.2.3"),
            Err("major is invalid".to_string())
        );
        assert_eq!("1".parse::<SemVer>(), Err("minor is missing".to_string()));

        assert_eq!(
            SemVer::from_str("1.2.x"),
            Err("patch is invalid".to_string())
        );

        assert_eq!(
            SemVer::from_str("1.2.3."),
            Err("invalid trailing input".to_string())
        );

        assert_eq!(
            SemVer::from_str("1.2.3"),
            Ok(SemVer {
                major: 1,
                minor: 2,
                patch: 3
            })
        );
    }
}
