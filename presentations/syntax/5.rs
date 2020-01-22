fn main() {
    let mut value = 0;
    // Loop with break
    loop {
        if value >= 10 {
            break;
        }
        value += 1;
    }
    // Break on conditional
    while value <= 10 {
        value += 1;
        // ...
    }
}