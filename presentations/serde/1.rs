#[macro_use] 

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    id: usize,
    direction: Direction,
}

#[derive(Debug, Serialize, Deserialize)]
enum Direction { North, South, East, West }

fn main() {}