fn main() {
    let first = 0..10;
    let second = 10..20;

    let (first_again, _): (Vec<_>, Vec<_>) = first
        .zip(second)
        .inspect(|x| println!("Inspect: {:?}", x))
        .unzip();

    for item in first_again {
        println!("{}", item);
    }
}