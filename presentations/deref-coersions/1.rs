struct Point {
    x: i32,
    y: i32
}

fn main() {
    let boxed_p = Box::new(Point { x: 1, y: 2 });
    println!("{}", boxed_p.x);
}