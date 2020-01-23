trait Foo {}

fn main() {
    use std::mem;

    println!("{}", mem::size_of::<&Foo>());
    println!("{}", mem::size_of::<Box<Foo>>());
}