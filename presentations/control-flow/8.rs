fn main() {
    let maybe_arg = std::env::args().nth(2);
    // can't know at compile time how many args are passed to our program
    if let Some(arg) = maybe_arg {
        println!("Got second command line argument: {}", arg);
    }
}
