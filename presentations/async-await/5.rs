use std::time::Duration;

use async_std::io;
use async_std::task;

// ... other function definitions

fn main() -> io::Result<()> {
    let duration = Duration::from_secs(5);
    // This async scope times out after 5 seconds.
    task::block_on(timeout_on_stdin(duration))
}
