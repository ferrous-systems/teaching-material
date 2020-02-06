use std::fs::File;
use std::io::Write;
use std::io;

fn main() -> io::Result<()> {
    let file_create = File::create("test");

    let file = match file_create {
        Ok(f) => f,
        Err(e) => panic!("File create failed: {}", e),
    };

    write_and_close(file) <1>
}

fn write_and_close(mut file: File) -> io::Result<()> { <1>
    file.write_all(b"Hello World!")

    <2>
}

