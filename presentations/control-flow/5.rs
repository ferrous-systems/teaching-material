fn main() {
    let mut iter = vec![1,2].into_iter();

    match iter.next() {
        Some(x) => println!("number: {}", x),
        None    => println!("No next item")
    }
}
