use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::sync::{Arc, Mutex};
use async_std::task;
use async_std::{io::BufReader, task::sleep};
use futures::channel::mpsc;
use log::{error, info};
use semver::EnumRepository;
use semver_api::{ApiError, Command};
use std::{convert::TryInto, env, io, str::FromStr, time::Duration};

fn main() -> io::Result<()> {
    pretty_env_logger::init();
    task::block_on(async {
        let (mut tx, mut rx) = mpsc::channel(5);
        let port = env::var("PORT").unwrap_or_else(|_| "7878".to_string());
        let addr = format!("127.0.0.1:{}", port);
        info!("serving at {}", addr);
        let listener = TcpListener::bind(addr).await?;

        let repository = Arc::new(Mutex::new(EnumRepository::new()));

        task::spawn(async move {
            let mut incoming = listener.incoming();
            while let Some(connection) = incoming.next().await {
                let stream = match connection {
                    Ok(stream) => stream,
                    Err(e) => {
                        error!("Connection error: {:?}", e);
                        continue;
                    }
                };

                let repository = repository.clone();
                match tx.try_send((repository, stream)) {
                    Ok(_) => {}
                    Err(e) => {
                        //let err: mpsc::TrySendError<(Arc<Mutex<EnumRepository>>, TcpStream)> = e;
                        if e.is_full() {
                            let (repository, mut stream) = e.into_inner();
                            let response: Result<String, _> =
                                semver_api::ApiResponse(Err(ApiError::OverCapacity)).try_into();
                            match response {
                                Ok(response) => {
                                    let _ = stream.write(response.as_bytes()).await;
                                }
                                Err(e) => {
                                    error!("Could not convert to API response: {:?}", e);
                                }
                            };
                        }
                    }
                }
                //tx.send((repository, stream)).await.ok();
            }
        });

        while let Some((repository, mut stream)) = rx.next().await {
            sleep(Duration::from_millis(500)).await;
            task::spawn(async move {
                let result = handle(&mut stream, &*repository).await;
                if result.is_err() {
                    error!("could not handle request: {:?}", result);
                }
                let response: Result<String, _> = semver_api::ApiResponse(result).try_into();
                match response {
                    Ok(response) => {
                        let _ = stream.write(response.as_bytes()).await;
                    }
                    Err(e) => {
                        error!("Could not convert to API response: {:?}", e);
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
