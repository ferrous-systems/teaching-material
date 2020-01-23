use std::time::Duration;

async fn timeout_on_stdin(duration: Duration) -> io::Result<()> {
    io::timeout(duration, read_from_stdin())
}