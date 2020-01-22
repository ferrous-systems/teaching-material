// &mut denotes a mutable borrow
fn accepts_borrow(thing: &mut u32) {
    *thing += 1
}

fn main() {
    let mut value = 1;
    accepts_borrow(&mut value);
    println!("{}", value)
}