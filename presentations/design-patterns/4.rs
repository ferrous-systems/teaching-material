pub struct Stuff {
    value: i64,
}

impl Stuff {
    /// constructor by convention
    fn new(value: i64) -> Self {
        Self { value: value }
    }
}
