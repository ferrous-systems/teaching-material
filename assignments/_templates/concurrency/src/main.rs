use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    solution();
}

fn problem() {
    let mut numbers = vec![0];

    for i in 0..10 {
        // compiler suggests move || but that doesn't work (first thread consumes)
        // creating one &mut numbers - one for each thread - also doesn't work (multiple &mut at the same time)
        thread::spawn(|| {
            numbers.push(i);
        });
    }
}

fn solution() {
    let numbers = Arc::new(Mutex::new(vec![])); // try using Rc instead of Arc - what happens?

    for i in 0..10 {
        let nr = numbers.clone();
        thread::spawn(move || {
            // same as (*nr).lock().unwrap() - why?
            let mut guard = nr.lock().unwrap();

            let numbers = &mut *guard;
            numbers.push(i);
        });
    }
    // bonus task: these numbers are in order - 0..9 -, so we're only concurrent, but not parallel!
    // how to fix this?
    dbg!(numbers);
}
