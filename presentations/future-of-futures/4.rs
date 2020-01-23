let (tx1, rx) = mpsc::channel();
let tx2 = tx1.clone();
let tx3 = tx1.clone();

let fut = future::ready(1)
    .then(move |x| {
        tx1.send(x).unwrap(); // Send 1
        tx1.send(2).unwrap(); // Send 2
        future::ready(3)
    }).map(move |x| {
        tx2.send(x).unwrap(); // Send 3
        tx2.send(4).unwrap(); // Send 4
        5
    }).map(move |x| {
        tx3.send(x).unwrap(); // Send 5
    }).run_in_background();
