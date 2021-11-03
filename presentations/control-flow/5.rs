fn main() {
    let will_overflow: Option<u8> = 10_u8.checked_add(250);
    match will_overflow {
        Some(sum) => println!("interesting: {}", sum),
        None => eprintln!("addition overflow!"),
    }
}
