use std::io::{Read, BufReader, BufRead, Lines};
use std::fs::File;

fn main() {

    let f = File::open("src/data/content.txt");

    let file = match f {
        Ok(file) => file,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };

    let mut buf_reader = BufReader::new(file);
    let mut content_string = String::new();
    buf_reader.read_to_string(&mut content_string).unwrap();
    println!("{}", content_string);
}
