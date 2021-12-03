/// This is like a 'struct FoobarContext;' in C
#[repr(C)]
pub struct FoobarContext { _priv: [i32; 0] }

extern "C" {
	fn foobar_init() -> *mut FoobarContext;
	fn foobar_do(ctx: *mut FoobarContext, foo: i32);
	fn foobar_destroy(ctx: *mut FoobarContext);
}

/// Use this in your Rust code
pub struct FoobarHandle(*mut FoobarContext);
