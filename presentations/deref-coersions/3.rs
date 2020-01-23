fn print_me(message: &str) { println!("{}", message); }

fn main() {
    print_me("Foo");
    let a_string = String::from("Bar");
    print_me(&a_string);
    print_me(a_string.as_str())
}