use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("test")?;

    print_filelen(&file)?;
    write_to_file(&mut file, data);
    print_filelen(&file)
}

fn print_filelen(f: &File) -> io::Result<()> {
    println!("len: {:?}", f.metadata()?.len());
    Ok(())
}

fn write_to_file(f: &mut File, data: &str) -> io::Result<()> {
    file.write_all(b"Hello World!")
}
