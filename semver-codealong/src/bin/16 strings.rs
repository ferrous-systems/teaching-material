struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

// TODO too much at once
impl From<&str> for SemVer {
    fn from(s: &str) -> Self {
        let parts = s
            .split(".")
            .filter_map(|b| b.parse().ok())
            .collect::<Vec<_>>();
        SemVer {
            major: parts[0],
            minor: parts[1],
            patch: parts[2],
        }
    }
}

fn main() {
    let sv: SemVer = "1.2.3".into();
}
