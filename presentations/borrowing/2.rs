#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let mut point = Point { x: 1, y: 2 };
    inspect(&point);
    move_point(&mut point, 3, 3);
    inspect(&point);
}

fn move_point(p: &mut Point, x: i32, y: i32) {
    p.x = x;
    p.y = y;
}

fn inspect(p: &Point) {
    println!("{:?}", p);
}