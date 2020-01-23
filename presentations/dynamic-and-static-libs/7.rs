#[no_mangle]
pub extern "C" fn inspect_point(p: *mut Point) {
    unsafe {
        let point: Box<Point> = Box::from_raw(p);
        point.inspect();
        std::mem::forget(point)
    };
}

#[no_mangle]
pub extern "C" fn inspect_point(p: &Point) {
    p.inspect();
}
