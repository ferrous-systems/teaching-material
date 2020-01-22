fn with_lifetimes<'a>(thing: &'a str) -> &'a str {
    thing
}

fn main() {
    let foo = "foo";
    println!("{}", with_lifetimes(foo))
}