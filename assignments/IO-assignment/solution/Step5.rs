use std::io::{Read, BufReader, BufRead, Lines};
use std::fs::File;

fn main() {

    let f = File::open("src/data/content.txt");

    let file = match f {
        Ok(file) => file,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };

    let mut buf_reader = BufReader::new(file).lines();

    for line in buf_reader {

        let line = match line {
            Ok(content) => content,
            Err(e) => panic!("Problem reading the line: {:?}", e),
        };

        println!("{}", line);
    }
}
