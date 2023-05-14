mod storage;
mod extractors;
mod pricetracker;

use reqwest::Client;
use select::document::Document;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let extractor_list: Vec<Box<dyn extractors::PageInfoExtractor>> = vec![
        Box::new(extractors::A8a{}),
        Box::new(extractors::Skalnik{}),
    ];

    let client = Client::new();
    let mut page_info_map: HashMap<String, pricetracker::PageInfo> = HashMap::new();

    let file_contents = std::fs::read_to_string("urls.txt")?;

    for url in file_contents.lines() {
        match extractor_list.iter().find(|e| {
            e.match_url(url)
        }) {
            Some(ext) => {
                let page_info = get_page_info(&client, url, ext.as_ref())
                    .await
                    .expect("failed getting page info");
                page_info_map.insert(url.to_string(), page_info);
            } 
            None => {
                println!("No extractor matched url: {}", url);
            }
        }

    }

    println!("{:#?}", page_info_map);

    Ok(())
}

async fn get_page_info(client: &Client, url: &str, ext: &dyn extractors::PageInfoExtractor) -> Result<pricetracker::PageInfo, Box<dyn std::error::Error>> {
    let response_text = client.get(url).send().await?.text().await?;
    let document = Document::from(response_text.as_str());
    ext.extract_page_info(document).map_err(Into::into)
}
