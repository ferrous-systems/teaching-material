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
    let mut program_list :Vec<Program> = vec![];

    // open "releases.txt", bail on error
    let mut releases = File::open("releases.txt")?;

    // use a `BufReader` to iterate over the lines of the file handle
    let mut br = BufReader::new(releases);

    // if the line can be read (it might be invalid data), split it on ","
    for line in br.lines() {
        if let Ok(raw) = line{
            let mut chunks = raw.split(",");
            // take the first element of your split - that's the name
            if let Some(name) = chunks.next(){
                // the rest is a list of &str slices that each can be MAPPED INTO a SemVer!
                let mut vers :Vec<SemVer> = vec![];
                for v in chunks {
                    vers.push(SemVer::from(v));
                }
                if vers.len() == 0{
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Malformed Line"));
                }
                // we're still in iterator land - time to collect and push the result to our program vec
                program_list.push(Program{name: name.to_string(), release_history: vers})
            }
        }
    }

    // finally, print the program vec.
    println!("{:?}",program_list);

    Ok(())
}
