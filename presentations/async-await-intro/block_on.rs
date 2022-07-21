use futures::executor::block_on;

async fn hello_world() {
    println!("This is my task!");
}

fn main() {
    let future = hello_world(); // Nothing is printed, yet
    block_on(future);
}
