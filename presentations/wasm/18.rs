#![feature(link_args)]

#[cfg_attr(target_arch="wasm32", link_args = "\
    --js-library site/utilities.js\
")]
extern {}

use std::os::raw::c_char;
use std::ffi::CStr;

extern {
    fn get_data() -> *mut c_char;
}

fn get_data_safe() -> String {
    let data = unsafe {
        CStr::from_ptr(get_data())
    };
    data.to_string_lossy()
        .into_owned()
}

fn main() {
    let data = get_data_safe();
    println!("{:?}", data);
}