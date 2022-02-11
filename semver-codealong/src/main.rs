use std::{convert::TryFrom, fmt::Display};
use std::{convert::TryInto, error::Error as StdError};

#[derive(Debug)]
struct ErrString(String);

impl Display for ErrString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl StdError for ErrString {}

impl ErrString {
    fn err() -> Box<Self> {
        Box::new(ErrString(
            "wrong format, need Major_u8.Minor_u8.Patch_u8".to_string(),
        ))
    }
}

#[derive(Debug, PartialEq)]
struct SemVer {
    major: u8,
    minor: u8,
    patch: u8,
}

type DynError = Box<dyn std::error::Error>;

impl TryFrom<&'_ str> for SemVer {
    type Error = DynError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<_> = value.split(".").collect();

        if parts.len() != 3 {
            return Err(ErrString::err());
        }

        let mut parts = parts.into_iter().map(|e| e.parse());
        let major = parts.next().unwrap()?;
        let minor = parts.next().unwrap()?;
        let patch = parts.next().unwrap()?;

        Ok(SemVer {
            major,
            minor,
            patch,
        })
    }
}

impl TryFrom<Vec<u8>> for SemVer {
    type Error = DynError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(ErrString::err());
        } else {
            Ok(SemVer {
                major: value[0],
                minor: value[1],
                patch: value[2],
            })
        }
    }
}

fn main() -> Result<(), DynError> {
    let sv = SemVer::try_from("1.2.3")?;

    println!("SemVer: {:?}", sv);

    let v = vec![0, 5, 17];
    let sv: SemVer = v.try_into()?;
    println!("SemVer: {:?}", sv);

    Ok(())
}
