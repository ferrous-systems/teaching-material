fn main() {
    let n = 2400;
    match (n % 400, n % 100, n % 4) {
        (0, _, _) => true,
        (_, 0, _) => false,
        (_, _, 0) => true,
        _ => false,
    }
}
