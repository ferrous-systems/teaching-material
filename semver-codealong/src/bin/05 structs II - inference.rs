// NOTE let participants recreate struct
struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

fn main() {
    // NOTE show how type changes automatically
    // NOTE two kinds of type annotation for primitives (on variable, or on the literal)
    // NOTE no default/uninit memory -- compiler error when not all fields specified
    // LATER b"strings"

    let major_number = 1;
    let minor = 2;

    let _version = SemVer {
        major: major_number,
        minor: minor,
        patch: 7,
    };
}
