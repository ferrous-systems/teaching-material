struct Container<'inner, T: 'inner> {
    inner: &'inner T
}