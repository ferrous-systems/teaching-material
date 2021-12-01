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
    // NOTE ownership, consume
    // NOTE stack vs heap (TODO: big jump!)
    // LINK https://doc.rust-lang.org/book/ch04-03-slices.html
    // TODO depending on participants' previous lang experience,
    // maybe branch out to "just quickly cover" vs "all the nitty gritty"
    // TODO expand, expand, expand

    // BROKEN why does this break?
    print_version(version);

    // NOTE Copy, Clone, hand object back and forth (possible - but unusual)
}

fn print_version(version: SemVer) {
    println!("we are at version {:?}", version);
}
