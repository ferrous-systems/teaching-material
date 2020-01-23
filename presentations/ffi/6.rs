extern "C" {
    pub fn register_callback(
        state: *mut c_void,
        name: *const c_char,
        cb: extern "C" fn(state: *mut c_void) -> *const c_uint,
    );
}