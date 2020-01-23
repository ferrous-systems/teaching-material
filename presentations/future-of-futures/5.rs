tokio::run_async(async {
    let client = Client::new();
    let uri = "http://httpbin.org/ip".parse().unwrap();

    let response = await!({
        client.get(uri).timeout(Duration::from_secs(10))
    }).unwrap();
    println!("Response: {}", response.status());

    let mut body = response.into_body();
    while let Some(chunk) = await!(body.next()) {
        let chunk = chunk.unwrap();
        println!("chunk = {}", str::from_utf8(&chunk[..]).unwrap());
    }
});