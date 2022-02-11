struct Point {
    x: i32,
    y: i32,
}

// error[E0106]: missing lifetime specifier
fn return_point() -> &Point {
    let p = Point { x: 1, y: 2 };
    &p
}

fn main() {
    return_point();
}
