use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = std::fs::File::open("test")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    drop(file);
    // do something long
    Ok(())
}
