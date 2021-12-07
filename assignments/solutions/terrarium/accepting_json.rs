


#[macro_use]



#[derive(Serialize,Deserialize)]
struct Person {
  name: String,
  age: u32,
  address: Address,
  phone_numbers: Vec<String>,
}

#[derive(Serialize,Deserialize)]
struct Address {
  street: String,
  city: String
}

use http_guest::{Request, Response};

pub fn user_entrypoint(req: &Request<Vec<u8>>) -> Response<Vec<u8>> {
  match *req.method() {
    http::method::POST => {
      let parsed_person = serde_json::from_slice::<Person>(req.body());

      match parsed_person {
          Ok(p) => {
            Response::builder()
              .status(200)
              .body(serde_json::to_vec(&p).unwrap())
              .unwrap()
          },
          Err(e) => {
            Response::builder()
              .status(422)
              .body(format!("Error occurred: {}", e).into())
              .unwrap()
          }
        }
      }
    },
    _ => {
        Response::builder().status(405)
          .header("allow", "POST")
          .body(vec![]).unwrap()
    }
  }
}

guest_app!(user_entrypoint);

