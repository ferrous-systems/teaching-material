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
    write_to_file(&mut file, data);
    print_filelen(&file)
}

fn print_filelen(f: &File) -> io::Result<()> {
    println!("len: {:?}", f.metadata()?.len());
    Ok(())
}

fn write_to_file(f: &mut File, data: &str) -> io::Result<()> {
    f.write_all(b"Hello World!")
}
