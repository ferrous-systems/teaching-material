use async_std::net::{TcpListener,TcpStream};
use async_std::prelude::*;
use async_std::sync::{Arc,Mutex};
use async_std::io::BufReader;
use async_std::io;
use async_std::task;

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
    task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:7878").await?;

        let storage = VecDeque::new();
        let rced_storage: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(storage));
    
        let mut incoming = listener.incoming();
        while let Some(connection) =  incoming.next().await {
            let mut stream = match connection {
                Ok(stream ) => stream,
                Err(e) => {
                    println!("Error occured: {:?}", e);
                    continue;
                }
            };
    
            let storage_handle = rced_storage.clone();
    
            task::spawn(async move {
                let res = handle(&mut stream, &storage_handle).await;
    
                if let Err(e) = res {
                    println!("Error occured: {:?}", e);
                }
            });
        };

        Ok(())
    })
}

async fn handle(stream: &mut TcpStream, mutex: &Mutex<VecDeque<String>>) -> Result<(), ServerError> {
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
    Ok(())
}

async fn read_command(stream: &mut TcpStream) -> Result<redisish::Command, ServerError> {
    let mut read_buffer = String::new();
    let mut buffered_stream = BufReader::new(stream);
    buffered_stream.read_line(&mut read_buffer).await?;
    let command = redisish::parse(&read_buffer)?;
    Ok(command)
}