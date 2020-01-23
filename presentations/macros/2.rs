macro_rules! implement_foo_for {
    [
        // This is a repetition!
        $($implement_for:ty,)*
    ] => {
        // This iterates over the repetition!
        $(impl Foo for $implement_for {})*
    }
}

implement_foo_for![u8, u16, u32, u64, usize,];
implement_foo_for! { i8, i16, i32, i64, isize, }
implement_foo_for!(f32, f64,);

trait Foo {
    fn foo(&self) {
        println!("Foo!");
    }
}

fn main() {
    1_u8.foo();
    1_u16.foo();
}