fn main() {
    // This is a simple future, sort of like a one-time channel. 
    // You get a (sender, receiver) when you invoke them.
    // Sending a value consumes that side of the channel.
    let (tx, rx) = oneshot::channel();

    // This consumes the sender, we can't use it afterwards.
    tx.send("Bears").unwrap();

    // Now we can wait for it to finish
    let result = rx.wait()
        .unwrap();
    println!("{}", result);
}

extern crate futures;
use futures::Future;
use futures::sync::oneshot;