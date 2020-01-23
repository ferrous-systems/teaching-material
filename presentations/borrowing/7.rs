fn call_me_maybe(maybe_number: &Option<&str>) {
    match *maybe_number {
        Some(ref s) => println!("calling {}", s),
        None => println!("no number, not calling")
    }
}

fn main() {
    let number = Some("+491728122469");
    call_me_maybe(&number);
}