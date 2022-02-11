use std::fmt::Display;

#[derive(Debug)]
struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

fn main() {
    let version = SemVer {
        major: 1,
        minor: 2,
        patch: 7,
    };

    println!("we are at version {:?}", version);
    println!("we are at version {}", version);
}
