use std::io::{Read, BufReader, BufRead, Lines};
use std::fs::File;


fn main() {

    let f = File::open("src/data/content.txt");

    match f {
        Ok(file) => println!("File opened!"),
        Err(e) => println!("Problem opening the file: {:?}", e),
    };
}
