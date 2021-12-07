#[macro_use]


use http_guest::{Request, Response, RequestExt};

pub fn user_entrypoint(req: &Request<Vec<u8>>) -> Response<Vec<u8>> {

    match *req.method() {
      http::Method::POST => {
        let backend_req = Request::builder()
            .method("POST")
            .uri(format!("https://httpbin.org/post"))
            .header("accept", "application/json")
            .body(req.body().clone())
            .unwrap();

        let pending = backend_req.send_async().unwrap();

        //....

        pending.wait().unwrap()
      }
      _ => {
          Response::builder().status(405)
          .header("allow", "POST")
          .body(vec![]).unwrap()
      }
    }

}


guest_app!(user_entrypoint);

