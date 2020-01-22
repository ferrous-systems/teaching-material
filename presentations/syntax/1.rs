// Function declaration
fn add_them(first: i32, second: i32) -> i32 {
    first + second
}

fn main() {
    // Mutable variable
    let mut some_value = 1;
    // Immutable, explict type
    let explicitly_typed: i32 = 1;
    
    // Function call
    some_value = add_them(some_value, explicitly_typed);
    
    // Macro, note the !
    println!("{}", some_value)
}