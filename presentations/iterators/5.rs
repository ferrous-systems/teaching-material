fn main() {
    // Limit the output to 30 to avoid infinite runtime.
    for item in (0..10).cycle().take(30) {
        println!("{}", item);
    }
}