use std::fs::{File, canonicalize};
use std::io::Write;

fn main() {
    let mut file = File::create("foo.txt").unwrap();
    file.write_all(b"Hello, world!").unwrap();
    
    let path = canonicalize("foo.txt").unwrap();
        
    let components: Vec<_> = path.components().collect();
    println!("{:?}", components);
}