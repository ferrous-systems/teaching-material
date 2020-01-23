#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let point = Point { x: 1, y: 1};
    let point_on_heap = Box::new(point);
    print_point(&point_on_heap);
}

fn print_point(p: &Point) {
    println!("{:?}", p);
}