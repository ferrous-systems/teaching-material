use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

fn main() -> std::io::Result<()> {
    let arg = std::env::args().nth(1);

    let message = match arg {
        Some(msg) => msg,
        None => String::from("Hello!"),
    };
    // or:
    // arg.unwrap_or(String::from("Hello!"));

    let mut stream = TcpStream::connect("127.0.0.1:7878")?;

    writeln!(stream, "{}", message)?;

    stream.shutdown(Shutdown::Write)?;

    let mut buffer = String::new();

    stream.read_to_string(&mut buffer)?;

    println!("{}", buffer);

    Ok(())
}
