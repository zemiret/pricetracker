mod config;
mod extractors;
mod pricetracker;
mod storage;

use std::{time::{SystemTime, UNIX_EPOCH}, process::Command};

use reqwest::Client;
use rusqlite::Connection;
use select::document::Document;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let extractor_list: Vec<Box<dyn extractors::PageInfoExtractor>> = vec![
        Box::new(extractors::A8a {}),
        Box::new(extractors::Skalnik {}),
    ];

    let conn = Connection::open("db.db3").expect("could not open DB");
    let watch_items = storage::list_watch_items(&conn).expect("could not list watch items");
    let client = Client::new();

    for watch_item in watch_items {
        let url = &watch_item.url;

        match extractor_list.iter().find(|e| e.match_url(url)) {
            Some(ext) => match get_page_info(&client, url, ext.as_ref()).await {
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

async fn get_page_info(
    client: &Client,
    url: &str,
    ext: &dyn extractors::PageInfoExtractor,
) -> Result<pricetracker::PageInfo, Box<dyn std::error::Error>> {
    // let response_text = client
        // .get(url)
    //     .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/116.0")
    //     .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
    //     .header("Accept-Language", "en,pl;q=0.7,en-US;q=0.3")
    //     .header("Referer", "https://8a.pl/liny-dynamiczne")
    //     .header("DNT", "1")
    //     .header("Connection", "keep-alive")
    //     .header("Cookie", "wp_customerGroup=NOT%20LOGGED%20IN; private_content_version=574dbdcd0aefa18c6db058fd1ad69b20; cf_clearance=E3YnCuKdG5IPw2LF3zyR8d1NV8xXHcEHoPn4IkNkEsA-1692208915-0-1-2cacc55a.c5987605.7bad9aba-0.2.1692208915; PHPSESSID=9hb0mdr42ai5vh4h6vsc62tuuj; srv=7; mage-cache-storage=%7B%7D; mage-cache-storage-section-invalidation=%7B%7D; mage-cache-sessid=true; mage-messages=; recently_viewed_product=%7B%7D; recently_viewed_product_previous=%7B%7D; recently_compared_product=%7B%7D; recently_compared_product_previous=%7B%7D; product_data_storage=%7B%7D; form_key=UEPm3wQ29PyMSolv; section_data_ids=%7B%22cart%22%3A1692208907%2C%22customer%22%3A1692210270%2C%22messages%22%3A1692210269%7D")
    //     .header("Upgrade-Insecure-Requests", "1")
    //     .header("Sec-Fetch-Dest", "document")
    //     .header("Sec-Fetch-Mode", "navigate")
    //     .header("Sec-Fetch-Site", "same-origin")
    //     .header("Sec-GPC", "1")
        // .send()
    //     .await?
    //     .text()
    //     .await?;


    // TODO: WEEEELLL, We are getting captcha when request goes through a reqwest library. Surprisingly, no captcha (so far) with curl


        let output = Command::new("curl")
        .arg("https://8a.pl/lina-wspinaczkowa-tendon-master-9-4-mm-80-m-green-red")
        .arg("--compressed")
        .arg("-H").arg("User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/116.0")
        // .arg("-H").arg("Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
        // .arg("-H").arg("Accept-Language: en,pl;q=0.7,en-US;q=0.3")
        // .arg("-H").arg("Accept-Encoding: gzip, deflate, br")
        // .arg("-H").arg("Referer: https://8a.pl/liny-dynamiczne")
        // .arg("-H").arg("DNT: 1")
        // .arg("-H").arg("Connection: keep-alive")
        // .arg("-H").arg("Cookie: wp_customerGroup=NOT%20LOGGED%20IN; private_content_version=574dbdcd0aefa18c6db058fd1ad69b20; cf_clearance=E3YnCuKdG5IPw2LF3zyR8d1NV8xXHcEHoPn4IkNkEsA-1692208915-0-1-2cacc55a.c5987605.7bad9aba-0.2.1692208915; PHPSESSID=9hb0mdr42ai5vh4h6vsc62tuuj; srv=7; mage-cache-storage=%7B%7D; mage-cache-storage-section-invalidation=%7B%7D; mage-cache-sessid=true; mage-messages=; recently_viewed_product=%7B%7D; recently_viewed_product_previous=%7B%7D; recently_compared_product=%7B%7D; recently_compared_product_previous=%7B%7D; product_data_storage=%7B%7D; form_key=UEPm3wQ29PyMSolv; section_data_ids=%7B%22cart%22%3A1692208907%2C%22customer%22%3A1692210270%2C%22messages%22%3A1692210269%7D")
        // .arg("-H").arg("Upgrade-Insecure-Requests: 1")
        // .arg("-H").arg("Sec-Fetch-Dest: document")
        // .arg("-H").arg("Sec-Fetch-Mode: navigate")
        // .arg("-H").arg("Sec-Fetch-Site: same-origin")
        // .arg("-H").arg("Sec-GPC: 1")
        .output()?;
    
    let mut response_text: String = "".to_string();
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);

        response_text = stdout.to_string();
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error executing curl: {}", stderr);
    }

    println!("{}", url);
    println!("{}", response_text);

    let document = Document::from(response_text.as_str());
    ext.extract_page_info(document).map_err(Into::into)
}
