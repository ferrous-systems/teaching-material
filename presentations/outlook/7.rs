trait Trait {}

impl Trait for i32 {}

// old
fn function1() -> Box<Trait> { }

// new
fn function2() -> Box<dyn Trait> { }