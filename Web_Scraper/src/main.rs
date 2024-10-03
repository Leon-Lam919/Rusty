use std::time::Instant;

use spider::tokio;
use spider::website::Website;

#[tokio::main]
async fn main() {
    let mut website: Website = Website::new("https://manganato.com");

    let start = Instant::now();
    website.crawl().await;
    let duration = start.elapsed();

    let links = website.get_links();

    for link in links.iter(){
        println!("-> {:?}", link.as_ref());
    }

    println!("Time elasped is {:?} for total pages: {:?}", duration, links.len());
}