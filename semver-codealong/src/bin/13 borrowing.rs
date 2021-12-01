#[derive(Debug)]
struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

fn main() {
    let mut version = SemVer {
        major: 1,
        minor: 2,
        patch: 7,
    };

    // NOTE introduce borrow visualization here
    // LINK https://github.com/rustviz/rustviz
    let version_ref = &mut version;
    print_version(&version);
    increment_version(version_ref);
    print_version(&version);
}

fn print_version(version: &SemVer) {
    println!("we are at version {:?}", version);
}

fn increment_version(version: &mut SemVer) {
    version.patch += 1;
}

// EXERCISE (group)
// - fn inc_major(&mut SemVer)
// - fn inc_major(&mut self)
