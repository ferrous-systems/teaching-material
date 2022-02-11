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

    // TODO add BROKEN tag?
    // NOTE code breaks intentionally, show broken version first,
    // then fix
    // NOTE this is to showcase the helpfulness of compiler errors
    version.patch = 8;
    version.patch += 1;
}
