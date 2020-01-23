struct Point {
    x: i32,
    y: i32
}

fn main() {
    let point = Point { x: 1, y: 1};
    let point_on_heap = Box::new(point);
}