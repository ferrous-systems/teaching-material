fn main() {
    let values = vec![1, 2, 3];
    println!("{:?}", values.get(0)); // Some(1)
    println!("{:?}", values.get(4)); // None
}