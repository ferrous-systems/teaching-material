fn main() {
    // Shorthand
    let value = Some(1).map(|v| v + 1);
    // With a block
    let value = Some(1).map(|v| {
        v + 1
    });
    // Explict return type
    let value = Some(1).map(|v| -> i32 {
        v + 1
    });
    // Declared
    let closure = |v| v + 1;
    let value = Some(1).map(closure);
}