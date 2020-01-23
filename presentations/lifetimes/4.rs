struct Container<T> {
    inner: &T
}

impl<T> Container<T> {
    fn borrow_inner(&self) -> &T {
        self.inner
    }
}

fn inner_drops_before_wrapper() {
    let n: i32 = 42;
    let w = Container { inner: &n };
    drop(n);
}


fn wrapper_drops_with_active_borrow() {
    let n: i32 = 42;
    let w = Container { inner: &n };
    let borrowed_n = w.borrow_inner();
    drop(w);
}