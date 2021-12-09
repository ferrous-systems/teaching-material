extern "C" {
    fn cool_library_function(x: u32, y: u32) -> u32;
}

fn main() {
    let result: u32 = unsafe {
        cool_library_function(6, 7)
    };
    println!("6 + 7 = {}", result);
}
