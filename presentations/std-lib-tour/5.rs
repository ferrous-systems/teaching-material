use std::io::{Read, Write};

fn main() {
    // File, Socket, Vector, ...
    let mut buffer: Vec<u8> = vec![];
    
    buffer.write(b"some bytes").unwrap();
    
    let mut read_in = String::new();
    buffer.as_slice().read_to_string(&mut read_in).unwrap();
    
    println!("{}", read_in);
}