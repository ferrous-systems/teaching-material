struct Container<T> {
    inner: &T,
}

impl<T> Container<T> {
    fn borrow_inner(&self) -> &T {
        self.inner
    }
}

fn inner_drops_before_container() {
    let s = "hello".to_string();
    let c = Container { inner: &s };
    drop(s);
}

fn container_drops_with_active_borrow() {
    let s = "hello".to_string();
    let c = Container { inner: &s };
    let borrowed_s = c.borrow_inner();
    drop(c);
}
