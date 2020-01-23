fn main() {
    let (tx, rx) = oneshot::channel();
    
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        tx.send("Bears").unwrap();
    });
    
    let result = rx.wait()
        .unwrap();
    println!("{}", result);
}

extern crate futures;
extern crate rand;

use std::thread;
use std::time::Duration;
use futures::Future;
use futures::sync::oneshot;