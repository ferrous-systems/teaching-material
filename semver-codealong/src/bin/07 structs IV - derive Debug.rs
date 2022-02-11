// NOTE many things can be derived, we'll see more later
// NOTE traits can be derived or manually implemented
// NOTE keep "trait" really short here - let's say this is an
// "aspect of functionality", similar to "interface" in other langs
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

    // NOTE go back to 02 println (with primitives)
    // - primitives and field access treated the same
    println!(
        "we are at version {}.{}.{}",
        version.major, version.minor, version.patch
    );

    // NOTE maybe also mention {:?}
    println!("we are at version {:?}", version);

    // LATER impl Display
}
