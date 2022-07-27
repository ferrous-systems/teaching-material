async fn subtask() {
    println!("> > subtask"); // <4>
}

async fn task() {
    println!("> Before subtask"); // <3>
    subtask().await;
    println!("> After subtask"); // <5>
}

fn main() {
    futures::executor::block_on(async {
        println!("before future"); // <1>
        let future = task();
        println!("future is created"); // <2>
        future.await;
        println!("future is awaited"); // <6>
    });
}
