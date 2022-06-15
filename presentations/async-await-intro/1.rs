async fn is_website_up(url: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let url = url.parse::<hyper::Uri>()?;

    let client = hyper::Client::new();
    let res = client.get(url).await?;

    let status_code = res.status();
    Ok(status_code.is_success())
}
