use tokio::runtime::Runtime; // Import the tokio runtime
use reqwest;
use scraper::{selector, ElementRef, Html, Selector};

async fn scrape(){
    let response = reqwest::get("https://manganato.com/").await.unwrap();
    let body = response.text().await.unwrap();
    let document = Html::parse_document(&body);
    let selector = Selector::parse("span").unwrap();

    let body_site_selector = Selector::parse("div.body-site").unwrap();
    let container_main_selector = Selector::parse("div.container.container-main").unwrap();
    let container_main_left_selector = Selector::parse("div.container-main-left").unwrap();
    let panel_content_homepage_selector = Selector::parse("div.panel-content-homepage").unwrap();
    let content_homepage_item_selector = Selector::parse("div.content-homepage-item").unwrap();
    let content_homepage_item_right_selector = Selector::parse("div.content-homepage-item-right").unwrap();
    let span_selector = Selector::parse("span.text-nowrap.item-author").unwrap();

    // Navigate through the nested elements
    for body_site in document.select(&body_site_selector) {
        for container_main in body_site.select(&container_main_selector) {
            for container_main_left in container_main.select(&container_main_left_selector) {
                for panel_content_homepage in container_main_left.select(&panel_content_homepage_selector) {
                    for content_homepage_item in panel_content_homepage.select(&content_homepage_item_selector) {
                        for content_homepage_item_right in content_homepage_item.select(&content_homepage_item_right_selector) {
                            for span in content_homepage_item_right.select(&span_selector) {
                                // Print the text content of the span element
                                println!("{}", span.text().collect::<Vec<_>>().concat());
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let rt = Runtime::new().unwrap();
    rt.block_on(scrape());
    Ok(())
}