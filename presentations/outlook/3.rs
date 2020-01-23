use some_library::foo;
use crate::foo::bar;

fn main() {
    foo(); // from `some_library`
    bar(); // from `foo` module
}