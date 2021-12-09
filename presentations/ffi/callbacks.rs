use std::os::raw::c_void;

pub type FooCallback = extern "C" fn(state: *mut c_void);

extern "C" {
    pub fn libfoo_register_callback(state: *mut c_void, cb: FooCallback);
}

extern "C" fn my_callback(_state: *mut c_void) {
    // Do stuff here
}

fn main() {
    unsafe { libfoo_register_callback(core::ptr::null_mut(), my_callback); }
}
