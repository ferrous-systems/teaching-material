use std::os::raw::c_uint;

extern "C" {
    fn cool_library_function(x: c_uint, y: c_uint) -> c_uint;
}

fn main() {
    let result: u32 = unsafe {
        cool_library_function(6, 7)
    };
    println!("{} should be 13", result);
}