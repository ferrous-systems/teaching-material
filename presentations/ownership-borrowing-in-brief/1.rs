use std::fs::File;
use std::io::Write;
use std::io;

fn main() -> io::Result<()> {
    let file_create = File::create("test");

    let mut file = match file_create {
        Ok(f) => f,
        Err(e) => panic!("File create failed: {}", e),
    };

    file.write_all(b"Hello World!")
}

