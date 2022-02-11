use calc::prelude::*;
use std::ffi::CStr;
use std::os::raw::c_char;

pub use calc::Expr;

#[repr(C)]
pub enum Result {
    Ok = 0,
    //... some errors
}

#[no_mangle]
pub extern "C" fn parse_and_eval(maybe_cstr: *const c_char, output: *mut i64) -> Result {
    // Validate input parameters
    todo!();

    // execute parse & eval
    todo!();

    // if successful, set output. Mind the ownership!
    todo!();
    Result::Ok
}

/// This will return null if unsuccessful
#[no_mangle]
pub extern "C" fn c_parse(maybe_cstr: *const c_char) -> *mut Expr {
    // validate input, return null pointer if invalid
    todo!();

    // return a raw pointer to an object whose ownership
    // has been given up on purpose (a "leak")
    // so that it can be freely used in the FFI caller
    // who later has to call our release function
    todo!();
}

#[no_mangle]
pub extern "C" fn c_eval(expr: *const Expr, output: *mut i64) -> isize {
    // validate `expr` and `output`
    todo!();

    // execute eval() and set output if successful
    todo!();
}

#[no_mangle]
/// release objects whose ownership was previously passed to C
pub extern "C" fn release_expr(box_expr: *mut Expr) {
    // take ownership back and go out of scope to drop
    todo!();
}
