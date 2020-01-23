fn main() {
    if this_can_fail(false).is_ok() {
        println!("It worked!");
    } else {
        println!("It didn't work!")
    }
}