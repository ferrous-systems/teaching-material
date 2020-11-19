use std::io;
use std::io::Read;

fn read_file(path: &std::path::Path) -> Result<String, io::Error> {
     let mut f = std::fs::File::open(path)?;

     let mut buffer = String::new();
     f.read_to_string(&mut buffer)?;

     Ok(buffer)
}
