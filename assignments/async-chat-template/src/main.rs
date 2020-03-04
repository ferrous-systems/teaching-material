use async_std::task;
use async_std::prelude::*;

use async_std::io;
use async_std::io::BufReader;

use async_std::net::{TcpStream, TcpListener};

use async_std::sync::channel;
use async_std::sync::{Sender,Receiver};

use std::collections::HashMap;

struct Client {
    name: String,
    sender: Sender<String>
}

struct Broker {
    clients: HashMap<String, Client>
}

enum ClientEvent {
   Connect(Client),
   Message { name: String, msg: String },
   Disconnect { name: String },
}

impl Broker {
    async fn receive_event(&mut self, event: ClientEvent) {
        match event {
            ClientEvent::Connect(c) => { self.clients.insert(c.name.clone(), c); },
            ClientEvent::Message { name: sender_name, msg } => {
                for (_, c) in self.clients.iter().filter(|(name, _)| **name != sender_name) {
                    c.sender.send(format!("{}: {}\n", sender_name, msg)).await 
                }
            },
            ClientEvent::Disconnect { name } => {
                self.clients.remove(&name);
            }
        }
    }
}

async fn broker_loop(mut incoming: Receiver<ClientEvent>) {
    let mut broker = Broker { clients: HashMap::new() };

    while let Some(event) = incoming.next().await {
        broker.receive_event(event);
    }
}

async fn client_loop(mut stream: TcpStream, broker_connection: Sender<ClientEvent>) -> io::Result<()> {

    // Accept a client
    // read its name line
    // register it with its broker

    let _incoming_task = task::spawn(async move {
        //
    });

    let _outgoing_task = task::spawn(async move {
        //
    });

    Ok(())
}

fn main() -> io::Result<()> {
    task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        println!("Listening on {}", listener.local_addr()?);

        let mut incoming = listener.incoming();

        let (broker_sender, broker_receiver) = channel(10);

        task::spawn(broker_loop(broker_receiver));

        while let Some(stream) = incoming.next().await {
            // fill in
        }
        Ok(())
    })
}
