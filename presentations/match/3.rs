fn main() {
    let maybe_file = std::fs::File::open("Not there!");

    match maybe_file {
        Ok(f) => { println!("File opened! Debug: {:?}", f) },
        Err(e) => { println!("File not opened!! Error: {:?}", e) }
    }
}