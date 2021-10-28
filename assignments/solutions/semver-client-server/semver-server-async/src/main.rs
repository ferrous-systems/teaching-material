use async_std::io::{BufReader, WriteExt};
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::sync::{Arc, Mutex};
use async_std::task;
use semver::EnumRepository;
use semver_api::{ApiError, Command};
use std::{convert::TryInto, env, io, str::FromStr};

fn main() -> io::Result<()> {
    task::block_on(async {
        let port = env::var("PORT").unwrap_or_else(|_| "7878".to_string());
        let addr = format!("127.0.0.1:{}", port);
        println!("serving at {}", addr);
        let listener = TcpListener::bind(addr).await?;

        let repository = Arc::new(Mutex::new(EnumRepository::new()));

        let mut incoming = listener.incoming();
        while let Some(connection) = incoming.next().await {
            let mut stream = match connection {
                Ok(stream) => stream,
                Err(_) => {
                    // unreachable as per documentation
                    continue;
                }
            };

            let repository = repository.clone();
            task::spawn(async move {
                let result = handle(&mut stream, &*repository).await;
                if result.is_err() {
                    eprintln!("Error occurred: {:?}", result);
                }
                let response: Result<String, _> = semver_api::ApiResult(result).try_into();
                match response {
                    Ok(response) => {
                        let _ = stream.write(response.as_bytes()).await;
                    }
                    Err(e) => {
                        eprintln!("Error occurred: {:?}", e);
                    }
                };
            });
        }

        Ok(())
    })
}

async fn handle(
    stream: &mut TcpStream,
    repository: &Mutex<EnumRepository>,
) -> Result<Option<String>, ApiError> {
    let command = read_command(stream).await?;

    let mut repository = repository.lock().await;
    let response = match command {
        Command::Get(crate_name) => {
            let crt = repository
                .get(&crate_name)
                .map_err(|e| ApiError::ParseError(format!("{:?}", e)))?;
            crt.try_into().map_err(|_| ApiError::Internal).map(Some)
        }
        Command::Put(crt) => {
            repository.insert(crt);
            Ok(None)
        }
        Command::Update(update) => {
            repository.add_release(update.crate_name, update.version)?;
            Ok(None)
        }
    };
    response
}

async fn read_command(stream: &mut TcpStream) -> Result<Command, ApiError> {
    let mut read_buffer = String::new();
    let mut buffered_stream = BufReader::new(stream);
    buffered_stream
        .read_line(&mut read_buffer)
        .await
        .map_err(|_| ApiError::InvalidCommand)?;
    Command::from_str(&read_buffer)
}
