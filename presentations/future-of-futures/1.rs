enum Poll<T> {
    Ready(T),
    Pending
}

trait Future {
    type Output;
    fn poll(self: PinMut<Self>, cx: &mut task::Context)
        -> Poll<Self::Output>;
}
