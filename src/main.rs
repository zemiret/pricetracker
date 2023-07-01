mod storage;
mod extractors;
mod pricetracker;

use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::Client;
use rusqlite::Connection;
use select::document::Document;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let extractor_list: Vec<Box<dyn extractors::PageInfoExtractor>> = vec![
        Box::new(extractors::A8a{}),
        Box::new(extractors::Skalnik{}),
    ];

    let conn = Connection::open("db.db3").expect("could not open DB"); 
    let watch_items = storage::list_watch_items(&conn).expect("could not list watch items");
    let client = Client::new();

    for watch_item in watch_items {
        let url = &watch_item.url;

        match extractor_list.iter().find(|e| {
            e.match_url(url)
        }) {
            Some(ext) => {
                let page_info = get_page_info(&client, url, ext.as_ref())
                    .await
                    .expect("failed getting page info");

                let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("duration_since(UNIX_EPOCH)").as_secs();
                storage::add_entry(&conn, page_info.price, now, watch_item.id).expect("error add_entry");
            } 
            None => {
                println!("No extractor matched url: {}", url);
            }
        }
    }

    let page_info_map = storage::get_entries(&conn).expect("storage.get_entries");

    // let mut page_info_map: HashMap<String, pricetracker::PageInfo> = HashMap::new();

    // let file_contents = std::fs::read_to_string("urls.txt")?;

    // for url in file_contents.lines() {
    //     match extractor_list.iter().find(|e| {
    //         e.match_url(url)
    //     }) {
    //         Some(ext) => {
    //             let page_info = get_page_info(&client, url, ext.as_ref())
    //                 .await
    //                 .expect("failed getting page info");
    //             page_info_map.insert(url.to_string(), page_info);
    //         } 
    //         None => {
    //             println!("No extractor matched url: {}", url);
    //         }
    //     }

    // }

    println!("{:#?}", page_info_map);

    Ok(())
}

async fn get_page_info(client: &Client, url: &str, ext: &dyn extractors::PageInfoExtractor) -> Result<pricetracker::PageInfo, Box<dyn std::error::Error>> {
    let response_text = client.get(url).send().await?.text().await?;
    let document = Document::from(response_text.as_str());
    ext.extract_page_info(document).map_err(Into::into)
}
