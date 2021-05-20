use async_std::net::{TcpListener,TcpStream};
use async_std::prelude::*;
use async_std::sync::{Arc,Mutex};
use async_std::io::BufReader;
use async_std::io;
use async_std::task;

use std::collections::VecDeque;
use std::time::Instant;

use tracing::Level;
use tracing_futures::Instrument;

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
    let subscriber = tracing_subscriber::fmt()
    .with_max_level(Level::TRACE)
    .finish();

    tracing::subscriber::set_global_default(subscriber)
      .expect("no global subscriber has been set");

    task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:7878").await?;

        let addr = listener.local_addr()?;
        tracing::event!(Level::INFO, listener.addr = addr.to_string().as_str(), "started listening to: {}", addr);

        let storage = VecDeque::new();
        let rced_storage: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(storage));
    
        let mut request_number: u64 = 0;

        let mut incoming = listener.incoming();
        while let Some(connection) =  incoming.next().await {
            let mut stream = match connection {
                Ok(stream ) => stream,
                Err(e) => {
                    println!("Error occurred: {:?}", e);
                    continue;
                }
            };
    
            let storage_handle = rced_storage.clone();
    
            request_number += 1;

            task::spawn(async move {
                let res = handle(&mut stream, &storage_handle)
                .instrument(tracing::span!(Level::INFO, "request", request = request_number)).await;
    
                if let Err(e) = res {
                    println!("Error occurred: {:?}", e);
                }
            });
        };

        Ok(())
    })
}

async fn handle(stream: &mut TcpStream, mutex: &Mutex<VecDeque<String>>) -> Result<(), ServerError> {
    let time = Instant::now();

    tracing::event!(Level::INFO, "inside request_handler!");

    let command = read_command(stream).await?;

    match command {
        redisish::Command::Publish(message) => {
            let mut storage = mutex.lock().await;
            storage.push_back(message);
        }
        redisish::Command::Retrieve => {
            let mut storage = mutex.lock().await;
            let data = storage.pop_front();
            match data {
                Some(message) => { stream.write(format!("{}", message).as_bytes()).await? },
                None => { stream.write(b"No message in inbox!\n").await? }
            };
        }
    }

    let elapsed = time.elapsed().as_micros().to_string();
    tracing::event!(Level::INFO, elapsed = elapsed.as_str(),  "request finished!");
    Ok(())
}

async fn read_command(stream: &mut TcpStream) -> Result<redisish::Command, ServerError> {
    let mut read_buffer = String::new();
    let mut buffered_stream = BufReader::new(stream);
    buffered_stream.read_line(&mut read_buffer).await?;
    let command = redisish::parse(&read_buffer)?;
    Ok(command)
}