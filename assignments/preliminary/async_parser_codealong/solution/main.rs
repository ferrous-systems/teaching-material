use std::time::Duration;

use async_parser_codealong::{
    random_packet, random_packet_channel_busy, random_packet_channel_lazy, raw_packets_stream,
    Handler,
};
use futures::{future::select_all, pin_mut, StreamExt};
use rand::thread_rng;
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinHandle,
    time::sleep,
};

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

fn memory_spam_task() -> JoinHandle<()> {
    let (_thread, mut random_source) = random_packet_channel_busy();
    tokio::spawn(async move {
        let mut handler = Handler::new("SPAM".to_string());
        while let Some(raw_packet) = random_source.recv().await {
            handler.handle_raw_packet(raw_packet);
            sleep(Duration::from_secs(10)).await;
        }
    })
}

fn good_task() -> JoinHandle<()> {
    let (tx, rx) = mpsc::unbounded_channel();
    let _thread = random_packet_channel_lazy(rx);
    tokio::spawn(async move {
        let mut handler = Handler::new("GOOD".to_string());
        loop {
            let (os_tx, os_rx) = oneshot::channel();
            match tx.send(os_tx) {
                Ok(_) => match os_rx.await {
                    Ok(raw_packet) => {
                        handler.handle_raw_packet(raw_packet);
                        sleep(Duration::from_secs(10)).await;
                    }
                    Err(_) => return,
                },
                Err(_) => return,
            }
        }
    })
}
