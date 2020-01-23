use std::thread;

#[derive(Debug)]
struct Thing;

// Can send between threads!
fn main() {
    let thing = Thing;
    
    thread::spawn(move || {
        println!("{:?}", thing);
    }).join().unwrap();
}