use async_std::io;
use std::time::Duration;

async fn timeout_on_stdin(duration: Duration) -> io::Result<()> {
    io::timeout(duration, async {
        let stdin = io::stdin();

        // Read a line from the standard input and display it.
        let mut line = String::new();
        stdin.read_line(&mut line).await?;
        dbg!(line);

        Ok(())
    })
}