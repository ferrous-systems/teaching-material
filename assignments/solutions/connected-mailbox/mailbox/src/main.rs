use std::net::{TcpListener,TcpStream};
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::VecDeque;

#[derive(Debug)]
enum ServerError {
    ParseError(redisish::Error),
    IoError(std::io::Error),
}

impl From<redisish::Error> for ServerError {
    fn from(e: redisish::Error) -> ServerError {
        ServerError::ParseError(e)
    }
}

impl From<std::io::Error> for ServerError {
    fn from(e: std::io::Error) -> ServerError {
        ServerError::IoError(e)
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut storage = VecDeque::new();

    for connection in listener.incoming() {
        let stream = match connection {
            Ok(stream) => stream,
            Err(e) => {
                println!("Error occurred: {:?}", e);
                continue;
            }
        };

        let res = handle(stream, &mut storage);

        if let Err(e) = res {
            println!("Error occurred: {:?}", e);
        }


    };

    Ok(())
}

fn handle(mut stream: TcpStream, storage: &mut VecDeque<String>) -> Result<(), ServerError> {
    let command = read_command(&mut stream)?;

    match command {
        redisish::Command::Publish(message) => {
            storage.push_back(message);
        },
        redisish::Command::Retrieve => {
            let data = storage.pop_front();

            match data {
                Some(message) => { 
                    write!(stream, "{}", message)?
                },
                None => { 
                    write!(stream, "No message in inbox!\n")?
                }
            }
        }
    }
    Ok(())
}

fn read_command(stream: &mut TcpStream) -> Result<redisish::Command, ServerError> {
    let mut read_buffer = String::new();
    let mut buffered_stream = BufReader::new(stream);
    buffered_stream.read_line(&mut read_buffer)?;
    let command = redisish::parse(&read_buffer)?;
    Ok(command)
}