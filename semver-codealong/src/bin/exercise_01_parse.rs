#![allow(unused)]

use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

impl SemVer {
    fn new(major: u16, minor: u16, patch: u16) -> SemVer {
        SemVer {
            major,
            minor,
            patch,
        }
    }

    fn new_short(major: u16) -> SemVer {
        Self::new(major, 0, 0)
    }

    fn info(&self) {
        println!(
            "hi, I'm a semver: {}.{}.{}",
            self.major, self.minor, self.patch
        )
    }

    fn patch(&mut self) -> &mut u16 {
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

impl From<String> for SemVer {
    fn from(s: String) -> Self {
        let vs: Vec<u16> = s.split(".").filter_map(|item| item.parse().ok()).collect();
        assert!(vs.len() == 3);
        SemVer {
            major: vs[0],
            minor: vs[1],
            patch: vs[2],
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_string() {
        let s: SemVer = "1.2.3".to_string().into();
        assert_eq!(s, SemVer::new(1, 2, 3));
    }

    #[test]
    #[should_panic]
    fn from_string_breaks() {
        let s: SemVer = "1.23".to_string().into();
        assert_eq!(s, SemVer::new(1, 2, 3));
    }
}
