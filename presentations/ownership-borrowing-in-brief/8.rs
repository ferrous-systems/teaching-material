use std::fs::File;

fn main() {
    let file = File::open("test").unwrap();
    let buffer = read_from(&file);
    drop(file);
    // do something long
}