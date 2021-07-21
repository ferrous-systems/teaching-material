#![allow(unused)]

use std::{
    collections::HashSet,
    convert::TryFrom,
    fmt::{Debug, Display},
    fs::File,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

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

impl From<&str> for SemVer {
    // string like "1.2.3"
    fn from(s: &str) -> Self {
        // |part| part.parse().ok() is equivalent to parse_and_convert_to_option
        let parts: Vec<_> = s.split(".").filter_map(|part| part.parse().ok()).collect();

        // bad form - better: use TryFrom
        assert!(parts.len() == 3);

        // advanced conversion directly from an array: use the `bytemuck` crate

        SemVer {
            major: parts[0],
            minor: parts[1],
            patch: parts[2],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Program {
    name: String,
    release_history: Vec<SemVer>,
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
    release_history: Vec<SemVer>,
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

pub trait Crate {
    fn name(&self) -> &str;
    fn latest_release(&self) -> Option<&SemVer>;
    fn append_release(&mut self, sv: SemVer) -> ();
}

impl Crate for Program {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn latest_release(&self) -> Option<&SemVer> {
        self.release_history.last()
    }

    fn append_release(&mut self, sv: SemVer) -> () {
        self.release_history.push(sv);
    }
}

impl Crate for Library {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn latest_release(&self) -> Option<&SemVer> {
        self.release_history.last()
    }

    fn append_release(&mut self, sv: SemVer) -> () {
        self.release_history.push(sv);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CrateKind {
    Library(Library),
    Program(Program),
}

impl Crate for CrateKind {
    fn name(&self) -> &str {
        match self {
            CrateKind::Library(lib) => lib.name(),
            CrateKind::Program(bin) => bin.name(),
        }
    }

    fn latest_release(&self) -> Option<&SemVer> {
        match self {
            CrateKind::Library(lib) => lib.latest_release(),
            CrateKind::Program(bin) => bin.latest_release(),
        }
    }

    fn append_release(&mut self, sv: SemVer) -> () {
        match self {
            CrateKind::Library(lib) => lib.append_release(sv),
            CrateKind::Program(bin) => bin.append_release(sv),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnumRegistry {
    pub crates: Vec<CrateKind>,
    names: HashSet<String>,
}

impl EnumRegistry {
    pub fn new() -> Self {
        EnumRegistry {
            crates: vec![],
            names: HashSet::default(),
        }
    }

    // - not deriving PartialEq!
    // - use trait! (show impl)
    pub fn contains(&self, crt: &CrateKind) -> bool {
        self.names.contains(crt.name())
    }

    pub fn add(&mut self, crt: CrateKind) {
        self.names.insert(crt.name().to_string());
        self.crates.push(crt);
    }

    pub fn get(&self, name: &str) -> Option<&CrateKind> {
        // optimization: if we don't have the crate at all, bail early
        // better optimization: use HashSet<CrateKind> instead - exercise for the reader ;)
        if self.names.get(name).is_none() {
            return None;
        }
        self.crates
            .iter()
            // .inspect(|c| println!("{}<< {}", name, c.name() == name))
            .find(|c| c.name() == name)
    }
}

#[test]
fn crate_lib() {
    let mut lib = Library::new("hello-lib");
    assert_eq!(lib.latest_release(), None);

    lib.append_release(SemVer::new_short(1));
    assert_eq!(lib.latest_release(), Some(&SemVer::new_short(1)));
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiError {
    InvalidCommand,
    // usually you'd wrap the underlying error and not serialize to string
    // here we serialize to remove the `Serialize` trait bound requirement on the nested error
    ParseError(String),
    NotFound,
    Internal,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ApiError::InvalidCommand => "invalid command",
            ApiError::ParseError(_) => "invalid payload",
            ApiError::NotFound => "not found",
            ApiError::Internal => "internal",
        };

        f.write_str(s)
    }
}

impl std::error::Error for ApiError {}
