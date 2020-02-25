use std::io::{Read, BufReader, BufRead, Lines};
use std::fs::File;

fn main() {

    let f = File::open("src/data/content.txt");

    let mut file = match f {
        Ok(file) => file,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };

    let mut content_string = String::new();
    file.read_to_string(&mut content_string).unwrap();
    println!("{}", content_string);
}
