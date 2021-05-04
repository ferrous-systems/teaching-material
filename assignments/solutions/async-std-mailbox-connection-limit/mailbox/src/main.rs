use async_std::net::{TcpListener,TcpStream};
use async_std::prelude::*;
use async_std::sync::{Arc,Mutex};
use async_std::io::BufReader;
use async_std::io;
use async_std::task;

use async_listen::{ListenExt, backpressure::Token, error_hint};

use std::collections::VecDeque;
use std::time;

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
    
        let mut incoming = listener.incoming()
            .log_warnings(log_accept_error)
            .handle_errors(time::Duration::from_millis(500))
            .backpressure(10);

        while let Some(mut connection) =  incoming.next().await {
            println!("connection accepted!");

            let storage_handle = rced_storage.clone();
    
            task::spawn(async move {
                println!("task started!");
                let res = handle(&mut connection, &storage_handle).await;
    
                if let Err(e) = res {
                    println!("Error occurred: {:?}", e);
                }
                println!("task ended!");

            });
        };

        Ok(())
    })
}

async fn handle(connection: &mut (Token, TcpStream), mutex: &Mutex<VecDeque<String>>) -> Result<(), ServerError> {
    task::sleep(time::Duration::from_millis(200)).await;
    let stream = &mut connection.1;
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

fn log_accept_error(e: &io::Error) {
    eprintln!("Accept error: {}. Sleeping 0.5s. {}", e, error_hint(&e));
}