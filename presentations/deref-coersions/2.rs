impl<T> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.inner
    }
}