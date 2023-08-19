mod config;
mod extractors;
mod pricetracker;
mod storage;
mod http_clients;

use std::{time::{SystemTime, UNIX_EPOCH}, fs, collections::HashMap};

use rusqlite::Connection;
use select::document::Document;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if config::DEBUG_MODE {
        println!("Run main, DEBUG_MODE=ON");
    }

    let extractor_list: Vec<Box<dyn pricetracker::PageInfoExtractor>> = vec![
        Box::new(extractors::A8a {}),
        Box::new(extractors::Skalnik {}),
    ];

    let conn = Connection::open("db.db3").expect("could not open DB");
    let watch_items = storage::list_watch_items(&conn).expect("could not list watch items");

    let client: Box<dyn pricetracker::HttpClient> = if config::DEBUG_MODE {
        let skalnik = fs::read_to_string("fixtures/skalnik.html").expect("cannot read skalnik.html");
        let a8a = fs::read_to_string("fixtures/a8a.html").expect("cannot read a8a.html");
        Box::new(http_clients::Mock::new(
            HashMap::from([
                (config::A8A_TEST_URL.to_string(), a8a),
                (config::SKALNIK_TEST_URL.to_string(), skalnik),
            ])
        )) as Box<dyn pricetracker::HttpClient>
    } else {
        Box::new(http_clients::Curl::new()) as Box<dyn pricetracker::HttpClient>
    };


    for watch_item in watch_items {
        let url = &watch_item.url;

        match extractor_list.iter().find(|e| e.match_url(url)) {
            Some(ext) => match get_page_info(client.as_ref(), url, ext.as_ref()).await {
                Ok(page_info) => {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("duration_since(UNIX_EPOCH)")
                        .as_secs();

                    println!("Adding entry from {}: {} (now: {})", ext.label(), url, now);
                    storage::add_entry(&conn, page_info.price, now, watch_item.id)
                        .expect("error add_entry");
                }
                Err(s) => println!("Error extracting from {}: {}", ext.label(), s),
            },
            None => {
                println!("No extractor matched url: {}", url);
            }
        }
    }

    let conn2 = Connection::open("db.db3").expect("could not open DB");
    let page_info_map = storage::get_entries(&conn2).expect("storage.get_entries");
    println!("{:#?}", page_info_map);

    Ok(())
}

async fn get_page_info(
    client: &dyn pricetracker::HttpClient,
    url: &str,
    ext: &dyn pricetracker::PageInfoExtractor,
) -> Result<pricetracker::PageInfo, Box<dyn std::error::Error>> {
    let response_text = client.get(url)?;
    let document = Document::from(response_text.as_str());
    ext.extract_page_info(document).map_err(Into::into)
}
