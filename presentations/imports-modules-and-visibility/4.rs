use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::open("foo.txt").unwrap();
    let mut buffer = [0; 10];

    f.read(&mut buffer).unwrap();

    println!("The bytes: {:?}", buffer);
}