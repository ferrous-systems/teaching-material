trait Bar {
    // This can be overridden
    fn default_implementation(&self) -> bool {
        true
    }
    fn required_implementation(&self);
}

impl Bar for Foo {
    fn required_implementation(&self) {
        // ...
    }
}

impl Foo {
    fn new() -> Self { Foo }
}

fn main() {
    let v = Foo::new();
    v.required_implementation();
    v.default_implementation();
}

struct Foo;