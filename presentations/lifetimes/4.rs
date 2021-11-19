struct Container<T> {
    inner: &T,
}

impl<T> Container<T> {
    fn borrow_inner(&self) -> &T {
        self.inner
    }
}

fn inner_drops_before_container() {
    let n: i32 = 42;
    let c = Container { inner: &n };
    drop(n);
}

fn container_drops_with_active_borrow() {
    let n: i32 = 42;
    let c = Container { inner: &n };
    let borrowed_n = c.borrow_inner();
    drop(w);
}
