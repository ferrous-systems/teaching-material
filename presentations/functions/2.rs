fn return_nothing() -> () {
    println!("Hello!");
}

fn return_max(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}