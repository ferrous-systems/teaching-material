fn main() {
    if let Err(e) = File::open("nein") {
        println!("{:?}", e);
    }
}