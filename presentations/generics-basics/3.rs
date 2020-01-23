enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let args = std::env::args;
    println!("{:?} {:?}", args().nth(0), args().nth(1));
}