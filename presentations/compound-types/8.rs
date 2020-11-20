enum Actions {
    StickAround,
    MoveTo { x: i32, y: i32},
}

fn main() {
    let action = Actions::MoveTo { x: 0, y: 0 };
}