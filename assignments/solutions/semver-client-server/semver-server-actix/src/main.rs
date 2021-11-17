//! Simple echo websocket server.
//! Open `http://localhost:8080/index.html` in browser
//! or [python console client](https://github.com/actix/examples/blob/master/websocket/websocket-client.py)
//! could be used for testing.

use std::{
    convert::TryInto,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use actix::prelude::*;
use actix_files as fs;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use log::{debug, error, info, warn};
use semver::EnumRepository;
use semver_api::{ApiError, ApiResponse, Command};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct ActixCommand(Command);

impl actix::Message for ActixCommand {
    type Result = ApiResponse;
}

/// do websocket handshake and start `MyWebSocket` actor
async fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<RepoServer>>,
) -> Result<HttpResponse, Error> {
    debug!("{:?}", r); // 👆 log::debug instead of println
    let res = ws::start(MyWebSocket::new(srv.get_ref().clone()), &r, stream);
    debug!("{:?}", res);
    res
}

/// websocket connection is long running connection, it easier
/// to handle with an actor
struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    addr: Addr<RepoServer>,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
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

                    self.addr
                        .send(ActixCommand(command))
                        .into_actor(self)
                        .then(|res, _, ctx| {
                            match res {
                                Ok(response) => {
                                    // for room in rooms {
                                    //     ctx.text(room);
                                    // }
                                    ctx.text(serde_json::to_string(&response).unwrap())
                                }
                                _ => error!("Something is wrong"),
                            }
                            fut::ready(())
                        })
                        .wait(ctx);

                    //ctx.text(serde_json::to_string(&response).unwrap());
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
    fn new(addr: Addr<RepoServer>) -> Self {
        Self {
            hb: Instant::now(),
            addr,
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

type Repo = Arc<Mutex<EnumRepository>>;
pub struct RepoServer {
    repo: Repo,
}

impl RepoServer {
    pub fn new(repo: Repo) -> RepoServer {
        RepoServer { repo }
    }
}

impl RepoServer {
    /// Send message to all users in the room
    fn send_message(&self, room: &str, message: &str, skip_id: usize) {
        // if let Some(sessions) = self.rooms.get(room) {
        //     for id in sessions {
        //         if *id != skip_id {
        //             if let Some(addr) = self.sessions.get(id) {
        //                 let _ = addr.do_send(Message(message.to_owned()));
        //             }
        //         }
        //     }
        // }
    }
}

impl Handler<ActixCommand> for RepoServer {
    type Result = MessageResult<ActixCommand>;

    fn handle(&mut self, msg: ActixCommand, _: &mut Context<Self>) -> Self::Result {
        let repository = &mut *self.repo.lock().unwrap();
        let response: Result<Option<String>, ApiError> = match msg.0 {
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

        MessageResult(ApiResponse(response))
    }
}

/// Make actor from `ChatServer`
impl Actor for RepoServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "semver-actix-websockets=debug,actix_server=info,actix_web=info",
    ); // 👆 dash vs underscore
    pretty_env_logger::init();

    let app_state: Repo = Default::default(); // 👆 derive this (pull repo)

    let server = RepoServer::new(app_state.clone()).start();

    HttpServer::new(move || {
        // 👆 move
        App::new()
            .data(app_state.clone())
            .data(server.clone())
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
