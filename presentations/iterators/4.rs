fn factorial(val: usize) -> usize {
    (1..val).rev().fold(1, |acc, x| acc * x)
}

fn main() {
    println!("{}", factorial(10));
}