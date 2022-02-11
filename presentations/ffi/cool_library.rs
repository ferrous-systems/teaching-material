use std::os::raw::{c_uint, c_char};

extern "C" {
    // We state that this function exists, but there's no definition.
    // The linker looks for this 'symbol name' in the other objects
    fn cool_library_function(x: c_uint, y: c_uint) -> c_uint;
}
