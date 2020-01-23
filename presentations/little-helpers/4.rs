#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let point = Box::new(Point { x: 1, y: 2});
    println!("{:?}", point);
}