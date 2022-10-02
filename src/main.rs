use anyhow::Result;
use fly_rust::{get_details, get_pages};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // let html = get_html("https://httpbin.org/get").await?;
    // let data = get_pages(1664667).await?;
    let data = get_details("/novel/1668078".to_owned()).await?;
    log::info!("{:?}", data);
    Ok(())
}
