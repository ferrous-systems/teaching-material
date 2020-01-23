fn main() {
    let evens = (0..10_000)
        .filter(|x| x % 2 == 0);
    for item in evens {
        println!("{}", item);
    }
}