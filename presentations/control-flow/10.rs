fn main() {
    let numbers = vec![1, 2, 3];
    // `for item in iterable` creates an iterator by calling `iterable.into_iter()`
    // and keeps calling `next() -> Option<Item>` on it until it receives `None`
    for num in numbers {
        println!("{}", num);
    }
}
