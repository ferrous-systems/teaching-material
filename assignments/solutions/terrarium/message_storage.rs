#[macro_use]

use http_guest::{KVStore, Request, Response};

pub fn user_entrypoint(kvs: &mut KVStore, req: &Request<Vec<u8>>) -> Response<Vec<u8>> {
    match *req.method() {
      http::Method::POST => {
          kvs.insert(req.uri().path(), req.body());
          Response::builder().status(200).body("Stored your message!".into()).unwrap()
      },
      http::Method::GET => {
        let maybe_msg = kvs.get(req.uri().path());

        match maybe_msg {
            Some(m) => Response::builder().status(200).body(m.into()).unwrap(),
            None => Response::builder().status(404).body("No message found".into()).unwrap()
        }
      }
      _ => {
          Response::builder().status(405).body("GET or POST only!".into()).unwrap()
      }
    } 
}

guest_app_kvs!(user_entrypoint);
