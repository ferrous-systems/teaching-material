fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async { 
        async_main().await 
    })
}

async fn async_main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // your async code
}
