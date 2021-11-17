mod chat_server;

use actix::{
    fut, Actor, ActorContext, ActorFuture, Addr, AsyncContext, ContextFutureSpawner, Handler,
    Running, StreamHandler, WrapFuture,
};
use actix_files as fs;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use chat_server::Join;
use log::{error, info, warn};

/// Define HTTP actor
struct WsChatSession {
    /// unique session id
    id: usize,
    /// The address of the chat server to communicate with
    server: Addr<chat_server::ChatServer>,
}

impl actix::Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    /// Actor has started
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        // send Join message to chat server, wait until chat server responds with session id
        self.server
            .send(Join {
                recipient: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, recipient, ctx| {
                match res {
                    Ok(session_id) => recipient.id = session_id,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    /// Actor is stopping the session
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // TODO send a Disconnect message to chat server to disconnect client

        Running::Stop
    }
}

/// Handle messages from chat server, forward to peer socket
impl Handler<chat_server::Message> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: chat_server::Message, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

/// Handler for ws::Message messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        info!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => (),
            Ok(ws::Message::Text(message)) => {
                // TODO send message to others
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
            }
            Ok(ws::Message::Continuation(_)) => (),
            _ => (),
        }
    }
}

/// Entry point to websocket route / chat
async fn chat_index(
    req: HttpRequest,
    stream: web::Payload,
    service: web::Data<Addr<chat_server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        WsChatSession {
            id: 0,
            server: service.get_ref().clone(),
        },
        &req,
        stream,
    );
    info!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let chat_server = chat_server::ChatServer::new().start();

    info!("Starting server");
    HttpServer::new(move || {
        App::new()
            .data(chat_server.clone())
            .service(web::resource("/ws/").route(web::get().to(chat_index)))
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
