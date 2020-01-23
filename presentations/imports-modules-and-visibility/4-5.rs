use std::{fs::File, io::{Read, Write}};

fn main() {
    let mut buffer = [0; 10];
    
    let mut f1 = File::open("foo.txt").unwrap();
    f1.read(&mut buffer).unwrap();

    let mut f2 = File::create("bar.txt").unwrap();
    f2.write_all(&buffer).unwrap();
}