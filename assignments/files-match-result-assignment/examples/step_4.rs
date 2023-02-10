use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn unwrap_file(open_result: Result<File, Error>) -> File {
    match open_result {
        Ok(file) => return file,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };
}

fn main() {
    let open_result = File::open("src/data/content.txt");

    let file = unwrap_file(open_result);

    let buf_reader = BufReader::new(file);

    let mut number = 0;

    for _line in buf_reader.lines() {
        number += 1;
    }

    println!("{}", number);
}
