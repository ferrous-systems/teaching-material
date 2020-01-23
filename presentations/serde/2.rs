#[macro_use] extern crate serde_derive;
extern crate serde_json;
use Direction::*;

fn main() {
    let action = Move { id: 1, direction: West };
    let payload = serde_json::to_string(&action);
    println!("{:?}", payload);
}

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    id: usize,
    direction: Direction,
}

#[derive(Debug, Serialize, Deserialize)]
enum Direction { North, South, East, West }