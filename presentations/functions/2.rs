fn return_nothing() {}

fn return_a_random() -> i32 {
    4 // Chosen by dice roll.
}

fn maybe_return_a_random(should: bool) -> Option<i32> {
    if should {
        Some(4)
    } else {
        None
    }
}
