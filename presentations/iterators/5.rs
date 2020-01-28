fn main() {
    let fizzbuzz = (0..10_000)
        .map(|x| match x {
            x if x % 15 == 0 => String::from("Fizzbuzz"),
            x if x % 3  == 0 => String::from("Fizz"),
            x if x % 5  == 0 => String::from("Buzz"),
            x => format!("{}", x),
        });
    for item in fizzbuzz {
        println!("{}", item);
    }
}