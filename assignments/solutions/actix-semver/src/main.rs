use std::{
    convert::TryInto,
    time::{Duration, Instant},
};

use actix::prelude::*;
use actix_files as fs;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use log::{debug, info, warn};
use semver::EnumRepository;
use semver_api::{ApiError, Command};
use serde_json::{Value, json};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// do websocket handshake and start `MyWebSocket` actor
async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    debug!("{:?}", r); // 👆 log::debug instead of println
    let res = ws::start(MyWebSocket::new(), &r, stream);
    debug!("{:?}", res);
    res
}

/// websocket connection is long running connection, it easier
/// to handle with an actor
struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    repo: EnumRepository,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

#[derive(actix::Message, Debug)]
#[rtype(result = "()")]
pub struct ResultMessage {
    message: String,
}

impl Handler<ResultMessage> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: ResultMessage, ctx: &mut Self::Context) -> Self::Result {
        let json = json!({"message": msg.message}).to_string();
        dbg!(&json);
        // ctx.text(serde_json::to_string(&json));
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        // process websocket messages
        debug!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            // 👆 ctx.text()/binary() send data to client
            Ok(ws::Message::Text(text)) => {
                // 👆 no trailing newline anymore - parsing has changed! (talk about API stuff, newline to terminate command, blah)
                dbg!(&text);
                if let Ok(command) = serde_json::from_str::<Command>(&text) {
                    // 👆 error handling (anyhow)
                    let repository = &mut self.repo;
                    let response: Result<Option<String>, ApiError> = match command {
                        Command::Get(crate_name) => {
                            repository
                                .get(&crate_name)
                                .map_err(|e| ApiError::ParseError(format!("{:?}", e)))
                                // 👇 either clone() or use serde_json::to_string
                                // 👇 unwrap is a bit meh, solution? --> Johann/Sebastian
                                .map(|crt| serde_json::to_string(crt).unwrap())
                                .map(Some)
                        }
                        Command::Put(crt) => {
                            repository.insert(crt);
                            Ok(None)
                        }
                        Command::Update(update) => repository
                            .add_release(update.crate_name, update.version)
                            .map_err(|_| ApiError::Internal)
                            .map(|_| None),
                        // 👆 useful conversion whoop whoop
                    };
                    ctx.text(serde_json::to_string(&response).unwrap());
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl MyWebSocket {
    fn new() -> Self {
        Self {
            hb: Instant::now(),
            repo: EnumRepository::default(), // 👆 derive this (pull repo)
        }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                warn!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "websocket_server=debug,actix_server=info,actix_web=info",
    ); // 👆 dash vs underscore
    pretty_env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // websocket route
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
            // static files
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    // start http server on 127.0.0.1:8080
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

