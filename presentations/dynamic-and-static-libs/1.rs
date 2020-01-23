#[derive(Debug)]
#[repr(C)]
pub struct Point {
    x: i32,
    y: i32
}

#[no_mangle]
pub extern "C" fn new_point(x: i32, y: i32) -> *mut Point {
    let p = Box::new(Point { x: x, y: y });
    Box::into_raw(p)
}

#[no_mangle]
pub extern "C" fn destroy_point(p: *mut Point) {
    unsafe { Box::from_raw(p) };
}

#[no_mangle]
pub extern "C" fn inspect_point(p: *mut Point) {
    unsafe {
        let point: Box<Point> = Box::from_raw(p);
        point.inspect();
    };
}
