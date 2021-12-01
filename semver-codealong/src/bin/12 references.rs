#[derive(Debug)]
struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

fn main() {
    let version = SemVer {
        major: 1,
        minor: 2,
        patch: 7,
    };

    print_version(&version);
    increment_version(&version);
    print_version(&version);
}

fn print_version(version: &SemVer) {
    println!("we are at version {:?}", version);
}

fn increment_version(version: &SemVer) {
    // BROKEN
    // NOTE immutable vs mutable refs
    // TERMINOLOGY shared vs exclusive more exact, but
    // sticking with (im)mutable for beginners - just briefly mention
    // shared/exclusive
    version.patch += 1;
}
