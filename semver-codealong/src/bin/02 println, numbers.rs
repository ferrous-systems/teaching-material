fn main() {
    // NOTE overflow, 32 bit, etc 
    // TODO granularity?
    let an_int = 123;
    let a_big_int = 1_000_000_000_000;
    let a_float = 3.14;
    let a_string = "Hello, string!";
    println!(
        "these are a few of my favorite things: {}, {}, {}, {}",
        an_int, a_big_int, a_float, a_string
    );
}
