use foo::foo;

mod foo {
    pub fn foo() {
        // ...
    }
}

// Will try to open `./bar.rs` relative to this file.
pub mod bar;

fn main() {
    foo()
}