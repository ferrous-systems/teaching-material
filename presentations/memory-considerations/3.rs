fn main() {
    use std::mem;

    println!("{}", mem::size_of::<&u8>());
    println!("{}", mem::size_of::<Box<u8>>());
}