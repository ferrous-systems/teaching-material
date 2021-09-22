use calc::prelude::*;
use std::ffi::CStr;
use std::os::raw::c_char;

// What I want to expose:
// * single function
// * takes a C string of "input"
// * returns (if successful) the output

// parse_and_eval
//
// Arguments?
//    * const char
// Return type?
//    * integer

// returns 0 if success, returns nonzero if failure
// on success, output is updated with the result?
//
// int parse_and_eval(char*, int64_t* output);

pub use calc::Expr;

#[repr(C)]
pub enum Result {
    Ok = 0,
    UnexpectedNullPointer,
    InvalidStringData,
    ParseError,
    EvalError,
}

#[no_mangle]
pub extern "C" fn parse_and_eval(maybe_cstr: *const c_char, output: *mut i64) -> isize {
    // Check if cstr is valid
    if maybe_cstr.is_null() {
        return Result::UnexpectedNullPointer;
    }
    let cstr = unsafe { CStr::from_ptr(maybe_cstr) };
    let string_data = match cstr.to_str() {
        Ok(s) => s,
        Err(_e) => return Result::InvalidStringData,
    };

    // check if output is non-null
    if output.is_null() {
        return Result::UnexpectedNullPointer;
    }

    // next: do parse and then eval

    let parsed = match parse(string_data) {
        Ok(p) => p,
        Err(_e) => return Result::ParseError,
    };

    // if successful, set error code, return
    let evaled = match eval(&parsed) {
        Ok(ev) => ev,
        Err(_e) => return Result::EvalError,
    };

    unsafe {
        *output = evaled;
    }

    Result::Ok
}

/// This will return null if unsuccessful
#[no_mangle]
pub extern "C" fn c_parse(maybe_cstr: *const c_char) -> *mut Expr {
    // Check if cstr is valid
    if maybe_cstr.is_null() {
        return std::ptr::null_mut();
    }
    let cstr = unsafe { CStr::from_ptr(maybe_cstr) };
    let string_data = match cstr.to_str() {
        Ok(s) => s,
        Err(_e) => return std::ptr::null_mut(),
    };

    match parse(string_data) {
        Ok(p) => {
            let ret_val: &'static mut Expr = Box::leak(Box::new(p));
            let ret_val_ptr = ret_val as *mut Expr;
            ret_val_ptr
        }
        Err(_e) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn c_eval(expr: *const Expr, output: *mut i64) -> isize {
    // Check if cstr is valid
    if expr.is_null() {
        return Result::UnexpectedNullPointer;
    }

    // check if output is non-null
    if output.is_null() {
        return Result::UnexpectedNullPointer;
    }

    // if successful, set error code, return
    let evaled = match eval(unsafe { &*expr }) {
        Ok(ev) => ev,
        Err(_e) => return Result::EvalError,
    };

    unsafe {
        *output = evaled;
    }

    Result::Ok
}

#[no_mangle]
pub extern "C" fn release_expr(box_expr: *mut Expr) {
    if !box_expr.is_null() {
        unsafe {
            let _box = Box::from_raw(box_expr);
        }
    }
}
