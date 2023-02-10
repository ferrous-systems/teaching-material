use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn unwrap_file(open_result: Result<File, Error>) -> File {
    match open_result {
        Ok(file) => return file,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };
}

fn content_to_string(mut file: File) -> Result<String, Error> {
    let mut content_string = String::new();
    file.read_to_string(&mut content_string)?;
    Ok(content_string)
}

fn main() {
    let open_result = File::open("src/data/content.txt");

    let file = unwrap_file(open_result);

    let content = content_to_string(file).unwrap();
    println!("{}", content);
}
