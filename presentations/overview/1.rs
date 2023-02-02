fn main() {
    let random_number = generate_random_number();
    let mut my_choice = 10;
    my_choice += random_number;
    println!("{my_choice}");
}

fn generate_random_number() -> i32 {
    4 // chosen by dice roll, guaranteed to be random
}