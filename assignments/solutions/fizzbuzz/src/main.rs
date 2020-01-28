fn main() {
    for i in 1..100 {
        println!("{}", fizzbuzz(i));
    }
}

fn fizzbuzz(i: u32) -> String {
    if i % 3 == 0 && i % 5 == 0 {
        format!("FizzBuzz")
    } else if i % 3 == 0 {
        format!("Fizz")
    } else if i % 5 == 0 {
        format!("Buzz")
    } else {
        format!("{}", i)
    }
}
