struct Point {
    x: i32,
    y: i32
}

fn return_point() -> Box<Point> {
    let p = Point { x: 1, y: 2 };
    Box::new(p)
}