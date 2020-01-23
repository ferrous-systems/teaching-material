use std::os::raw::{c_uint, c_char};
use std::ffi::CStr;

extern "C" { cool_printf_wrapper(strr: const *c_char) -> c_uint; }

fn main() {
    unsafe {
        let my_strr = "Hello to the World!";
        let _: u32 = cool_printf_wrapper(CStr::from(my_strr).unwrap());
    }
}