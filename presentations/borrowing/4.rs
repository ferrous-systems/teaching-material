fn main() {
    let number: &mut i32 = &mut 4;
    *number = 10;
    println!("{}", number);
}