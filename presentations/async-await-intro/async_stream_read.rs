let mut lines = async_buffered_text_stream.lines();
while let Some(line) = lines.next().await {
    if let Ok(line) = line {
        // line is a valid utf8 `String`
        process_line(line);
    }
}
