//! Simple websocket client.
use std::{str::from_utf8, time::Duration};

use actix::io::SinkWrite;
use actix::*;
use actix_codec::Framed;
use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
    BoxedSocket, Client,
};
use bytes::Bytes;
use futures::stream::{SplitSink, StreamExt};
use semver::{Crate, Program, SemVer};
use semver_api::{ApiResponse, Command, Update};

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    pretty_env_logger::init();

    let sys = System::new("websocket-client");

    Arbiter::spawn(async {
        let (response, framed) = Client::new()
            .ws("http://127.0.0.1:8080/ws/")
            .connect()
            .await
            .map_err(|e| {
                println!("Error: {}", e);
            })
            .unwrap();

        println!("{:?}", response);
        let (sink, stream) = framed.split();
        let addr = RepoClient::create(|ctx| {
            RepoClient::add_stream(stream, ctx);
            RepoClient(SinkWrite::new(sink, ctx))
        });

        let program_name = "hello_bin".to_string();
        let program = Program::new(program_name.clone());

        let commands = vec![
            Command::Put(Crate::Program(program)),
            Command::Update(Update {
                crate_name: "ertjwjbrkwrkerbwkhrba".to_string(),
                version: SemVer::new_short(1),
            }),
            Command::Update(Update {
                crate_name: program_name.clone(),
                version: SemVer::new_short(1),
            }),
            Command::Update(Update {
                crate_name: program_name.clone(),
                version: SemVer::new_short(2),
            }),
            Command::Update(Update {
                crate_name: program_name.clone(),
                version: SemVer::new_short(2),
            }),
            Command::Get(program_name.clone()),
        ];

        for command in commands {
            addr.do_send(ClientCommand(command));
        }

        // start console loop
        /*
        thread::spawn(move || loop {
            let mut cmd = String::new();
            if io::stdin().read_line(&mut cmd).is_err() {
                println!("error");
                return;
            }

            match serde_json::from_str::<Command>(&cmd) {
                Ok(command) => {
                    println!("COMMAND: {:?}", command);
                    addr.do_send(ClientCommand(command));
                }
                Err(err) => println!("Failed to parse command '{}'", err),
            }
        });
        */
    });
    sys.run().unwrap();
}

struct RepoClient(SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>);

#[derive(Message)]
#[rtype(result = "()")]
struct ClientCommand(semver_api::Command);

impl Actor for RepoClient {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        // start heartbeats otherwise server will disconnect after 10 seconds
        self.hb(ctx)
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        println!("Disconnected");

        // Stop application on disconnect
        System::current().stop();
    }
}

impl RepoClient {
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(Duration::new(1, 0), |act, ctx| {
            act.0.write(Message::Ping(Bytes::from_static(b"")));
            act.hb(ctx);

            // client should also check for a timeout here, similar to the
            // server code
        });
    }
}

/// Handle stdin commands
impl Handler<ClientCommand> for RepoClient {
    type Result = ();

    fn handle(&mut self, msg: ClientCommand, _ctx: &mut Context<Self>) {
        let text = serde_json::to_string(&msg.0).unwrap();
        self.0.write(Message::Text(text));
    }
}

/// Handle server websocket messages
impl StreamHandler<Result<Frame, WsProtocolError>> for RepoClient {
    fn handle(&mut self, msg: Result<Frame, WsProtocolError>, _: &mut Context<Self>) {
        if let Ok(Frame::Text(txt)) = msg {
            if let Ok(string) = from_utf8(&txt) {
                if let Ok(response) = serde_json::from_str::<ApiResponse>(string) {
                    println!("Server: {:?}", response)
                }
            }
        }
    }

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Connected");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("Server disconnected");
        ctx.stop()
    }
}

impl actix::io::WriteHandler<WsProtocolError> for RepoClient {}
