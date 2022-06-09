select! {
    example = fetch_into_string("http://example.com") =>
        println!("Example is faster with body:\n{}", example?),
    forever = fetch_into_string("http://httpforever.com") =>
        println!("HttpForever is faster with body:\n{}", forever?),
}
