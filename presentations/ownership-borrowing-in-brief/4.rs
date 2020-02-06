use std::fs::File;
use std::io::Write;
use std::io;

fn main() -> io::Result<()> {
    let file_create = File::create("test");

    let mut file = match file_create {
        Ok(f) => f,
        Err(e) => panic!("File create failed: {}", e),
    };

    print_filelen(&file)?;
    file.write_all(b"Hello World!")?;
    print_filelen(&file)
}

fn print_filelen(f: &File) -> io::Result<()> {
    println!("len: {:?}", f.metadata()?.len());
    Ok(())
}
