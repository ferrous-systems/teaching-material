trait Log<T> {
    fn log(&self, t: T);
}

impl<T> Log<T> for T where T: Debug {
    fn log(&self, t: T) {
        println!("Logging: {:?}", t);
    }
}