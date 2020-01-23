fn main() {
    let maybe_file = std::fs::File::open("Not there!");

    match maybe_file {
        Err(e) => { println!("Error: {:?}", e) }
        _ => {}
    }
}