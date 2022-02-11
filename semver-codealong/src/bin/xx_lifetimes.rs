#[derive(Debug, Default)]
struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

struct Crate<'c> {
    name: String,
    release_history: Vec<&'c SemVer>,
}

fn main() {
    let version = SemVer::default();

    let release_history = vec![&version];

    let c = Crate {
        name: "hello".to_string(),
        release_history,
    };
}
