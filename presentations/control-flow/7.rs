fn main() {
    let mut iter = vec![1, 2, 3].into_iter();

    match iter.next() {
        Some(x) if x % 2 == 0 => println!("even number!"),
        _                     => println!("not even"),
    }
}
