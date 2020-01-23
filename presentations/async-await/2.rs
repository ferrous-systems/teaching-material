use async_std::io;

async fn read_from_stdin() -> io::Result<()> {
    let stdin = io::stdin();

    // Read a line from the standard input and display it.
    let mut line = String::new();
    stdin.read_line(&mut line).await?;
    dbg!(line);

    Ok(())
}