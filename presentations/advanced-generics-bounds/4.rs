trait Logger<X: Debug> {
    fn log(&self, x: X);
}