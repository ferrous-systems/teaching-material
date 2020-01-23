const NUM_OF_TASKS: usize = 10;

fn main() {
    let mut rx_set = Vec::new();

    for index in 0..NUM_OF_TASKS {
        let (tx, rx) = futures::oneshot();
        rx_set.push(rx);

        thread::spawn(move || {
            println!("{} --> START", index);
            sleep_a_little_bit();
            tx.send(index).unwrap();
            println!("{} <-- END", index);
        });
    }

    // `join_all` lets us join together the set of futures.
    let result = join_all(rx_set)
        .wait()
        .unwrap();

    println!("{:?}", result);
}

extern crate rand;
extern crate futures;

use std::thread;
use futures::Future;
use futures::future::join_all;
use std::time::Duration;
use rand::distributions::{Range, IndependentSample};

// This function sleeps for a bit, then returns how long it slept.
pub fn sleep_a_little_bit() -> u64 {
    let mut generator = rand::thread_rng();
    let possibilities = Range::new(0, 1000);

    let choice = possibilities.ind_sample(&mut generator);

    let a_little_bit = Duration::from_millis(choice);
    thread::sleep(a_little_bit);
    choice
}