struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

impl SemVer {
    // NOTE purely convention
    fn new(major: u16, minor: u16, patch: u16) -> SemVer {
        SemVer {
            major,
            minor,
            patch,
        }
    }
}
fn main() {
    let major_number = 1;
    let minor = 2;

    // NOTE return type of constructor: SemVer or Self
    let _version = SemVer::new(major_number, minor, 5);
}
