struct Empty;

struct WithFields {
    foo: i32,
    bar: Choice,
}

type Explanation = String;

enum Choice {
    Yes,
    No,
    Maybe(Explanation),
}

fn main() {}