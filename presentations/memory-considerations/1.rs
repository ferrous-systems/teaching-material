fn main() {
    use std::mem;

    let i: i32 = 42;

    println!("{}", mem::size_of::<i32>());
    println!("{}", mem::size_of_val(&i));
}