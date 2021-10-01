use async_parser_codealong::{random_packet, raw_packets_stream, Handler};
use futures::{future::select_all, pin_mut, StreamExt};
use rand::thread_rng;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    // console_subscriber::init();

    let bad_task = async_blocking_task();
    //let good_task = good_task();

    let _ = select_all([bad_task /* , good_task */]).await;
}

// previously:
async fn do_stream() {
    let mut handler = Handler::new("STREAM".to_string());
    let packet_data = raw_packets_stream();
    pin_mut!(packet_data);
    while let Some(raw_packet) = packet_data.next().await {
        handler.handle_raw_packet(raw_packet);
    }
}

fn async_blocking_task() -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut handler = Handler::new("BLOCK".to_string());
        loop {
            let raw_packet = random_packet(thread_rng());
            handler.handle_raw_packet(raw_packet);
        }
    })
}
