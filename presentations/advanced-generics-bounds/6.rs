use std::fmt::Debug;

struct Wrapper<T> {
    inner: T
}

impl<T> Wrapper<T> {
    fn new(inner: T) -> Wrapper<T> where T: Debug {
        Wrapper { inner: inner }
    }

    fn inspect(&self) where T: Debug {
        println!("{:?}", &self.inner);
    }
}