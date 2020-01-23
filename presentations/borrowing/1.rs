#[derive(Debug)]
struct Point {
    x: i32,
    y: i32 
}

fn main() {
    let mut point = Point { x: 1, y: 2 };
    inspect(&point);
    point.x = 2;
    inspect(&point);
}

fn inspect(p: &Point) {
    println!("{:?}", p);
}