use tokio::runtime::Runtime; // Import the tokio runtime
use reqwest;

async fn scrape() {
    let response = reqwest::get("https://www.rust-lang.org").await.unwrap();
    let body = response.text().await.unwrap();

    println!("body = {body:?}");
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let rt = Runtime::new().unwrap();
    rt.block_on(scrape());
    Ok(())
}