use std::os::raw::{c_uint, c_char};

extern "C" {
    cool_printf_wrapper(strr: const *c_char) -> c_uint;
}