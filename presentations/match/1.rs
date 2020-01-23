fn main() {
    let mut args = std::env::args();

    match args.nth(1) {
        Some(arg) => { println!("Argument: {}", arg)},
        None => { println!("No Argument") }
    }
}