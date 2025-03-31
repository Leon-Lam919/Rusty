use tokio::main;
use std::time::Instant;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::Write;
use anyhow::{Context, Result};
use reqwest::{Client, ClientBuilder};
use tokio::time::{sleep, Duration};
use rand::Rng;

struct Page {
    html: String,
}

impl Page {
    fn content(&self) -> &str {
        &self.html
    }
}

struct Website {
    base_url: String,
    pages: Vec<Page>,
    client: Client,
}

impl Website {
    fn new(base_url: &str) -> Result<Self> {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .build()?;

        Ok(Website {
            base_url: base_url.to_string(),
            pages: Vec::new(),
            client,
        })
    }

    async fn crawl(&mut self) -> Result<()> {
        let mut next_url = Some(self.base_url.clone());
        while let Some(ref url) = next_url {
            match self.fetch_page(&url).await {
                Ok(body) => {
                    self.pages.push(Page { html: body.clone() });
                    let document = Html::parse_document(&body);
                    let next_selector = Selector::parse("a[href=https//manganato.com/genre-all]").unwrap();
                    next_url = document
                        .select(&next_selector)
                        .next()
                        .and_then(|element| element.value().attr("href"))
                        .map(|href| href.to_string());
                    
                    // Random delay between requests
                    let delay = rand::thread_rng().gen_range(5..15);
                    sleep(Duration::from_secs(delay)).await;
                },
                Err(e) => {
                    eprintln!("Error fetching {}: {}. Retrying...", url, e);
                    continue;
                }
            }
        }
        Ok(())
    }

    async fn fetch_page(&self, url: &str) -> Result<String> {
        const MAX_RETRIES: u32 = 3;
        let mut retries = 0;
        
        while retries < MAX_RETRIES {
            match self.client.get(url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        return Ok(response.text().await?);
                    }
                },
                Err(e) => {
                    eprintln!("Request error: {}. Retrying...", e);
                }
            }
            
            retries += 1;
            let backoff_duration = Duration::from_secs(2u64.pow(retries));
            sleep(backoff_duration).await;
        }
        
        Err(anyhow::anyhow!("Max retries reached for URL: {}", url))
    }

    fn get_pages(&self) -> Result<&Vec<Page>> {
        Ok(&self.pages)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    let mut website = Website::new("https://manganato.com/genre-all")?;
    website.crawl().await?;
    let duration = start.elapsed();
    let selector_title = Selector::parse("a.genres-item-img.bookmark_check").unwrap();
    let selector_view = Selector::parse("span.genres-item-view").unwrap();
    let mut file = File::create("./output.txt").context("Failed to create output file")?;
    let pages = website.get_pages()?;
    for page in pages.iter() {
        let content = page.content();
        let document = Html::parse_document(content);
        for element in document.select(&selector_title) {
            if let Some(title) = element.value().attr("title") {
                println!("Title: {}", title);
                writeln!(file, "Title: {}", title).context("Failed to write to file")?;
            }
        }
        for element in document.select(&selector_view) {
            let texts = element.text().collect::<String>().trim().to_string();
            println!("Views: {}", texts);
            writeln!(file, "Views: {}", texts).context("Failed to write to file")?;
        }
    }
    println!("Duration of scrape: {:?}", duration);
    Ok(())
}
