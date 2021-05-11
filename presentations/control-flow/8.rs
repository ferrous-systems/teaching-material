fn main() {
    let maybe_arg = std::env::args().nth(2);

    if let Some(arg) = maybe_arg {
        println!("Command line argument passed: {}", arg);
    }
}
