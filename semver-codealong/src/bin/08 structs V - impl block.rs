#[derive(Debug)]
struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

impl SemVer {
    // NOTE talk about &ref later in more detail
    // NOTE also about consuming self
    fn info(&self) {
        println!(
            "I am at version {}.{}.{}",
            self.major, self.minor, self.patch
        );
    }
}

fn main() {
    let version = SemVer {
        major: 1,
        minor: 2,
        patch: 7,
    };

    println!("we are at version {:?}", version);

    // NOTE first explain Debug, then vvvv this
    println!(
        "we are at version {}.{}.{}",
        version.major, version.minor, version.patch
    );

    version.info();

    // LATER impl Display
}
