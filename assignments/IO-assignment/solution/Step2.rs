use std::io::{Read, BufReader, BufRead, Lines};
use std::fs::File;


fn main() {

    let f = File::open("src/data/content.txt");

    match f {
        Ok(file) => println!("{}"),
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };
}
