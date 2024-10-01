use spider::tokio;
use spider::website::Website;

#[tokio::main]
async fn main() {
    let mut website: Website = Website::new("https://manganato.com/");

    website.crawl().await;

    let links = website.get_pages();

    if let Some(links) = links {
        for link in links.iter() {
            println!("- {:?}", link);
        }
    }
}