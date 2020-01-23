fn accept_either<S>(thing: S) -> String
where S: AsRef<str> {
    String::from("foo") + thing.as_ref()
}

fn main() {
    println!("{}", accept_either("blah"));
    println!("{}", accept_either(String::from("blah")));
}