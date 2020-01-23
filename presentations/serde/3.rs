#[macro_use] extern crate serde_derive;
extern crate serde_json;

fn main() {
    let payload = "{\"id\":1,\"direction\":\"West\"}";
    let action = serde_json::from_str::<Move>(&payload);
    println!("{:?}", action);
}

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    id: usize,
    direction: Direction,
}

#[derive(Debug, Serialize, Deserialize)]
enum Direction { North, South, East, West }