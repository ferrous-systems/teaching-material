struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

#[derive(PartialEq, Eq)]
enum Maturity {
    Alpha,
    Beta,
    Stable,
}

impl SemVer {
    fn maturity(&self) -> Maturity {
        if self.major == 0 {
            if self.minor == 0 {
                Maturity::Alpha
            } else {
                Maturity::Beta
            }
        } else {
            Maturity::Stable
        }
    }
}

fn main() {
    let version = SemVer {
        major: 1,
        minor: 2,
        patch: 7,
    };

    if version.maturity() == Maturity::Stable {
        println!("looks stable to me!");
    }
}
