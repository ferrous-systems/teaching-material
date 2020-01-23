enum Movement {
    Right(i32),
    Left(i32),
    Up(i32),
    Down(i32),
}

fn main() {
    let movement = Movement::Left(12);
}