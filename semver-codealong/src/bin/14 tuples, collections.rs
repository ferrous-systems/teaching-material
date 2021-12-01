// !UNFINISHED
// IMPL
// - fn destructure (introduce tuples first)

#[derive(Debug)]
struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

// TODO too much at once here
impl From<[u16; 3]> for SemVer {
    fn from(arr: [u16; 3]) -> Self {
        SemVer {
            major: arr[0],
            minor: arr[1],
            patch: arr[2],
        }
    }
}

fn main() {
    // NOTE first show arrays
}
