use std::collections::HashMap;

use actix::{Actor, Context, Handler, Recipient};
use log::{error, info, warn};
use rand::{prelude::ThreadRng, Rng};

type SessionId = usize;

#[derive(actix::Message, Debug)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(actix::Message, Debug)]
#[rtype(SessionId)]
pub struct Join {
    /// The recipient that joined the chat
    pub recipient: Recipient<Message>,
}

#[derive(actix::Message, Debug)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// The session id from client
    pub session_id: SessionId,
    /// The message from the client
    pub message: String,
}

/// TODO add a Disconnect message that removes the session from the chat server
/// similar to `Join`.
pub struct Disconnect {}

pub struct ChatServer {
    /// The list of current sessions
    sessions: HashMap<SessionId, Recipient<Message>>,
    /// Random number generator
    rng: ThreadRng,
}

impl ChatServer {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    /// Send message to all users of the chat (broadcast)
    fn send_message(&self, message: &str, session_id: SessionId) {
        info!("Broadcast message '{}'", message);

        for (id, session) in &self.sessions {
            // do not send message back to sender
            if *id != session_id {
                let _ = session.do_send(Message(message.to_owned()));
            }
        }
    }
}

/// Makes the ChatServer an Actor
impl Actor for ChatServer {
    type Context = Context<Self>;
}

/// Implement the Message handler to add new client to chat
impl Handler<Join> for ChatServer {
    type Result = SessionId;

    /// New user connects to chat
    fn handle(&mut self, msg: Join, _ctx: &mut Self::Context) -> Self::Result {
        info!("Someone joined the chat.");

        // generate new session id and add the session to list
        let session_id = self.rng.gen::<usize>();
        self.sessions.insert(session_id, msg.recipient);

        // notify all recipient someone joined the room
        self.send_message("Someone joined", session_id);

        session_id
    }
}

/// Implement the Message handler to broadcast a text message from a client
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    /// The message is simply forwarded to the chat server
    fn handle(&mut self, msg: ClientMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.send_message(&msg.message, msg.session_id);
    }
}
