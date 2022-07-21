async fn call_some_api() {}

async fn do_interesting_things() {
    println!("I'm an async function and I do interesting things");
    call_some_api().await
}

fn main() {
    do_interesting_things(); // <1>
    async {
        do_interesting_things().await; // <2>
    };
}
