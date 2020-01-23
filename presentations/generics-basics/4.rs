enum Result<T, E> {
    Ok(T),
    Err(E)
}

fn main() {
    let file = std::fs::File::open("I don't exist!");
    println!("{:?}", file);
}