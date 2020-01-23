const BUFFER_SIZE: usize = 57;

fn main() {
    // We're using a bounded channel here with a limited size.
    let (mut tx, rx) = mpsc::channel(BUFFER_SIZE);

    thread::spawn(move || {
        for index in 0..(BUFFER_SIZE+2) {
            sleep_a_little_bit();
            // When we `send()` a value it consumes the sender. Returning
            // a 'new' sender which we have to handle. In this case we just
            // re-assign.
            match tx.send(index).wait() {
                // Why do we need to do this? This is how back pressure is implemented.
                // When the buffer is full `wait()` will block.
                Ok(new_tx) => tx = new_tx,
                Err(_) => panic!("Oh no!"),
            }
        }
        // Here the stream (`tx`) is dropped, completing it.
    });

    // We can `.fold()` like we would an iterator. In fact we can do many
    // things like we would an iterator.
    let sum = rx.fold(0, |acc, val| {
            // Notice when we run that this is happening after each item of
            // the stream resolves, like an iterator.
            println!("--- FOLDING {} INTO {}", val, acc);
            // `ok()` is a simple way to say "Yes this worked."
            // `err()` also exists.
            ok(acc + val)
        })
        .wait()
        .unwrap();
    println!("Sum: {}", sum);
}

extern crate rand;
extern crate futures;

use std::time::Duration;
use std::thread;
use rand::distributions::{Range, IndependentSample};
use futures::future::{Future, ok};
use futures::stream::Stream;
use futures::sync::mpsc;
use futures::Sink;

// This function sleeps for a bit, then returns how long it slept.
pub fn sleep_a_little_bit() -> u64 {
    let mut generator = rand::thread_rng();
    let possibilities = Range::new(0, 100);

    let choice = possibilities.ind_sample(&mut generator);

    let a_little_bit = Duration::from_millis(choice);
    thread::sleep(a_little_bit);
    choice
}