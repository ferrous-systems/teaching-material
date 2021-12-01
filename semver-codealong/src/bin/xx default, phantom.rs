use std::marker::PhantomData;
#[derive(Debug, Default)]
struct S<T> {
    i: i32,
    p: PhantomData<T>,
}
fn main() {
    println!("Hello, world!");
    let s: S<u8> = S {
        i: 5,
        ..Default::default()
    };
}
