async fn fetch_into_string(url: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let url: hyper::Uri = url.parse()?;

    let client = hyper::Client::new();
    let res = client.get(url).await?;

    let bytes = hyper::body::to_bytes(res.into_body()).await?;
    Ok(String::from_utf8(bytes.to_vec())?)
}
