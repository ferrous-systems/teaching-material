#![allow(unused)]

use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Read},
};

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

impl From<&str> for SemVer {
    fn from(s: &str) -> Self {
        let vs: Vec<u16> = s.split(".").filter_map(|item| item.parse().ok()).collect();
        assert!(vs.len() == 3);
        SemVer {
            major: vs[0],
            minor: vs[1],
            patch: vs[2],
        }
    }
}

#[derive(Debug)]
struct Program {
    name: String,
    release_history: Vec<SemVer>,
}

fn main() -> Result<(), std::io::Error> {
    // create a `Vec` to hold the list of programs

    // open "releases.txt", bail on error

    // use a `BufReader` to iterate over the lines of the file handle

    // if the line can be read (it might be invalid data), split it on ","

    // take the first element of your split - that's the name

    // the rest is a list of &str slices that each can be MAPPED INTO a SemVer!

    // we're still in iterator land - time to collect and push the result to our program vec

    // finally, print the program vec.

    Ok(())
}
