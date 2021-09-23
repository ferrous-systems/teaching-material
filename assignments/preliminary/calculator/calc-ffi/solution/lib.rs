use calc::prelude::*;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::panic::catch_unwind;

pub use calc::Expr;

#[repr(C)]
pub enum Result {
    Ok = 0,
    UnexpectedNullPointer,
    InvalidStringData,
    ParseError,
    EvalError,
    Fatal,
}

#[no_mangle]
pub extern "C" fn parse_and_eval(maybe_cstr: *const c_char, output: *mut i64) -> Result {
    let res = catch_unwind(|| {
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
    });

    match res {
        Ok(inner) => inner,
        Err(_) => Result::Fatal,
    }
}

/// This will return null if unsuccessful
#[no_mangle]
pub extern "C" fn c_parse(maybe_cstr: *const c_char) -> *mut Expr {
    let res = catch_unwind(|| {
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
            Ok(p) => Box::into_raw(Box::new(p)),
            Err(_e) => std::ptr::null_mut(),
        }
    });

    match res {
        Ok(inner) => inner,
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn c_eval(expr: *const Expr, output: *mut i64) -> Result {
    let res = catch_unwind(|| {
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
    });

    match res {
        Ok(inner) => inner,
        Err(_) => Result::Fatal,
    }
}

#[no_mangle]
pub extern "C" fn release_expr(box_expr: *mut Expr) {
    if !box_expr.is_null() {
        unsafe {
            let _box = Box::from_raw(box_expr);
        }
    }
}
