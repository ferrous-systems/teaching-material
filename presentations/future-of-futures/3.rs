use futures::channel::oneshot;
let (tx1, rx1) = oneshot::channel::<i32>();

let t = thread::spawn(|| tx1.send(1).unwrap());
rx1.map_ok(move |x| println!("Received: {}", x)).run_in_background();
t.join().unwrap();