use std::rc::Rc;

struct Point {
   x: i32,
   y: i32,
}

fn main() {
    let rced_point = Rc::new(Point { x: 1, y: 1});
    let first_handle = rced_point.clone();
    let weak = Rc::downgrade(&first_handle);
}