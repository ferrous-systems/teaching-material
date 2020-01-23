use std::rc::Rc;
use std::thread;

// Does not work!
fn main() {
    let only_one_thread = Rc::new(true);

    thread::spawn(move || {
        println!("{:?}", only_one_thread);
    }).join().unwrap();
}