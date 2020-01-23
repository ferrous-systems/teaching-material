use std::io;
use std::fs::File;
use std::io::prelude::*;

enum MyError {
    FileWriteError
}

impl From<io::Error> for MyError {
    fn from(e: io::Error) -> MyError {
        MyError::FileWriteError
    }
}

fn write_to_file_using_try() -> Result<(), MyError> {
    let mut file = try!(File::create("my_best_friends.txt"));
    try!(file.write_all(b"This is a list of my best friends."));
    println!("I wrote to the file");
    Ok(())
}
// This is equivalent to:
fn write_to_file_using_match() -> Result<(), MyError> {
    let mut file = try!(File::create("my_best_friends.txt"));
    match file.write_all(b"This is a list of my best friends.") {
        Ok(v) => v,
        Err(e) => return Err(From::from(e)),
    }
    println!("I wrote to the file");
    Ok(())
}