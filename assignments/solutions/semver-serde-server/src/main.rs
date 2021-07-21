use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

use semver::ApiError;
use semver::{CrateKind, EnumRegistry};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut repository = EnumRegistry::new();

    for connection in listener.incoming() {
        let mut stream = match connection {
            Ok(stream) => stream,
            Err(e) => {
                println!("Error occurred: {:?}", e);
                continue;
            }
        };

        let res = handle(&mut stream, &mut repository);

        if let Err(e) = res {
            write!(stream, "{}", e)?;
            println!("Error occurred: {:?}", e);
        }
    }

    Ok(())
}

enum Command {
    Get(String),
    Put(CrateKind),
}

fn handle(stream: &mut TcpStream, repository: &mut EnumRegistry) -> Result<(), ApiError> {
    let command = read_command(stream)?;

    match command {
        Command::Get(crate_name) => {
            let crt = repository.get(&crate_name);
            match crt {
                Some(crt) => {
                    write!(
                        stream,
                        "OK {}",
                        serde_json::to_string(crt).map_err(|_| ApiError::Internal)?
                    )
                    .map_err(|_| ApiError::Internal)?;
                }
                None => {
                    write!(stream, "NOT FOUND").map_err(|_| ApiError::Internal)?;
                }
            }
        }
        Command::Put(crt) => {
            repository.add(crt);
            write!(stream, "OK").map_err(|_| ApiError::Internal)?;
        }
    }
    Ok(())
}

fn read_command(stream: &mut TcpStream) -> Result<Command, ApiError> {
    let mut read_buffer = String::new();
    let mut buffered_stream = BufReader::new(stream);
    buffered_stream
        .read_line(&mut read_buffer)
        .map_err(|_| ApiError::InvalidCommand)?;

    let command_without_newline = read_buffer.split_once("\n");
    if command_without_newline.is_none() {
        return Err(ApiError::ParseError("missing newline".to_string()));
    }

    let (command_without_newline, _) = command_without_newline.unwrap();
    if let Some((command_str, payload)) = command_without_newline.split_once(" ") {
        let command = match command_str {
            "GET" => {
                //let mut name = payload.to_string();
                // remove trailing newline
                //let _ = name.pop();
                Command::Get(payload.to_string())
            }
            "PUT" => {
                let crt: CrateKind = serde_json::from_str(payload).map_err(|serde_error| {
                    ApiError::ParseError(format!("serde: {:?}", serde_error))
                })?;
                Command::Put(crt)
            }
            _ => return Err(ApiError::InvalidCommand),
        };
        Ok(command)
    } else {
        Err(ApiError::ParseError("nonsense".to_string()))
    }
}
