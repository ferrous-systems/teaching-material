#[derive(Debug)]
struct Dot {
    x: i32,
    y: i32
}

fn main() {
    let dot = Dot { x: 1, y: 2 }; // <1>
    pacman(dot);
}

fn pacman(dot: Dot) { // <2>
    println!("Eating {:?}", dot);
    // <3>
}
