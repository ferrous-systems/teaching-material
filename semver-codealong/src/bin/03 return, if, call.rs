fn main() {
    let result = say_hi("hello");
    if result {
        println!("it was hello!");
    }
}

fn say_hi(s: &str) -> bool {
    println!("I got a string: {}", s);
    if s == "hello" {
        let v: Vec<_> = vec![1];
        // NOTE mention that semicolon here is an error
        true
    } else {
        false
    }
}
