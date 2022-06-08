use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {

    let open_result = File::open("src/data/content.txt");

    let file = match open_result {
        Ok(file) => file,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };

    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {

        let line = match line {
            Ok(content) => content,
            Err(e) => panic!("Problem reading the line: {:?}", e),
        };

        println!("{}", line);
    }
}
