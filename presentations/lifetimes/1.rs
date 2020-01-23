struct Point {
    x: i32,
    y: i32
}

fn return_point<'a>() -> &'a Point {
    let p = Point { x: 1, y: 2};
    &p
}

fn main() {
    return_point();
}