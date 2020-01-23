use std::sync::Mutex;

fn main() {

    let mut mutex = Mutex::new(0);
    
    // Use a new scope to force a drop.
    {
        let mut val = mutex.get_mut().unwrap();
        *val += 1;
    }
    
    println!("{}", *mutex.lock().unwrap());
}