use std::error::Error;

use tokio::{join, select};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let is_example_up = is_website_up("http://example.com").await?;
    println!("Example.com is up? {}", is_example_up);

    let check_example = is_website_up("http://example.com");
    let check_forever = is_website_up("http://httpforever.com");

    let (example_result, forever_result) = join!(check_example, check_forever);
    println!("example is up: {}", example_result?);
    println!("forever is up: {}", forever_result?);

    select! {
        example = fetch_into_string("http://example.com") =>
            println!("Example is faster with body:\n{}", example?),
        forever = fetch_into_string("http://httpforever.com") =>
            println!("HttpForever is faster with body:\n{}", forever?),
    }

    Ok(())
}

async fn is_website_up(url: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let url: hyper::Uri = url.parse()?;

    let client = hyper::Client::new();

    let res = client.get(url).await?;

    let status_code = res.status();
    Ok(status_code.is_success())
}

async fn fetch_into_string(url: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let url: hyper::Uri = url.parse()?;

    let client = hyper::Client::new();
    let res = client.get(url).await?;

    let bytes = hyper::body::to_bytes(res.into_body()).await?;
    Ok(String::from_utf8(bytes.to_vec())?)
}
