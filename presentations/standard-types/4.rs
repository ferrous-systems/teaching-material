fn main() {
    let maybe_a_value = Some(1);
    match maybe_a_value {
        Some(v) => println!("{}", v),
        None    => println!("None"),
    }
}