struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

fn main() {
    // NOTE use RA "fill in fields"
    // NOTE mention allow_unused ("attributes")
    // NOTE mention primitive types
    // NOTE no constructor
    // NOTE all fields are required, no uninit mem
    // TODO expand/more notes on memory safety and/or refer to SLIDE deck
    let version = SemVer {
        major: 1,
        minor: 2,
        patch: 7,
    };
}
