use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
    let mut buffer = String::new();

    stream.read_to_string(&mut buffer)?;

    println!("string: {}", buffer);

    stream.write(buffer.as_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}
