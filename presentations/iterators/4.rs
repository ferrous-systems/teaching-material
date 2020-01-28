fn main() {
    let items = vec![0, 1, 2];
    let mut iterator = items.into_iter();
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
}