struct Byte {
    field: u8
}

fn main() {
    use std::mem;

    println!("{}", mem::size_of::<Byte>());

    println!("{}", mem::size_of::<[Byte; 2]>());
}