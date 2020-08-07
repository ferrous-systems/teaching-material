use tracing_future::*;

use async_std::task;
use std::thread;
use std::time;

fn main() {
    task::block_on(async {
        let async_sleep = async {
            task::sleep(time::Duration::from_millis(100)).await;
        };
    
        task::spawn(poll_max(async_sleep)).await;
    
        let sync_sleep = async {
            thread::sleep(time::Duration::from_millis(100));
        };
    
        task::spawn(poll_max(sync_sleep)).await;

        let async_sleep = async {
            task::sleep(time::Duration::from_millis(100)).await;
        };

        task::spawn(deadline(time::Duration::from_millis(10), async_sleep)).await;
    });
}