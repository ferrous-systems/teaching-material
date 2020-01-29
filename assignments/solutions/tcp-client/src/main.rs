use std::io::prelude::*;
use std::net::{TcpStream,Shutdown};

fn main() -> std::io::Result<()> {
    let arg = std::env::args().nth(1);

    let message = match arg {
        Some(msg) => msg,
        None => String::from("Hello!"),
    };

    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    stream.write(message.as_bytes())?;

    stream.shutdown(Shutdown::Write)?;

    let mut buffer = String::new();

    stream.read_to_string(&mut buffer)?;

    println!("{}", buffer);

    Ok(())
}
