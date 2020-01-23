use std::fmt::Debug;

fn main() {
    let pointer_to_int = &mut 1;
    let raw = pointer_to_int as *mut i32;
    unsafe { deref_pointer(raw) };
}

unsafe fn deref_pointer<T: Debug>(p: *mut T) {
    println!("{:?}", *p)
}
