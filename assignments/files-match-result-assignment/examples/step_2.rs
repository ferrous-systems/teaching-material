use std::fs::File;
use std::io::Error;

// first function

fn unwrap_file(open_result: Result<File, Error>) -> File {
    match open_result {
        Ok(file) => return file,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };
}

fn main() {
    let open_result = File::open("src/lib/content.txt");

    let _file = unwrap_file(open_result);
}
