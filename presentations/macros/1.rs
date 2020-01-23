macro_rules! double { 
  // Input parameters
  ($value:expr)
  =>
  // Output
  ($value * 2);
}

fn main() {
    let doubled = double!(5);
    println!("{}", doubled);
    // Alternatives:
    double! { 5 };
    double![5];
}