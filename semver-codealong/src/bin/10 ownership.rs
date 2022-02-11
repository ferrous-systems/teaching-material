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

    print_version(version);
}

fn print_version(version: SemVer) {
    println!("we are at version {:?}", version);
}
