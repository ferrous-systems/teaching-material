fn main() {
    match this_can_fail(false) {
        Ok(val) => println!("Success: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}