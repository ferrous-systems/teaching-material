use std::fs::File;
use std::io::Write;
use std::io;

fn main() -> io::Result<()> {
    let file_create = File::create("test"); // <1>

    let mut file = match file_create {  // <2>
        Ok(f) => f, // <3>
        Err(e) => panic!("File create failed: {}", e),
    };

    file.write_all(b"Hello World!")
}  // <4>
