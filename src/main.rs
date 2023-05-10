use reqwest::Client;
use select::document::Document;
use select::predicate::{Name, Class, Descendant};
use std::collections::HashMap;
use std::num::ParseFloatError;

fn price_from_string(s: &str) -> Result<f32, ParseFloatError> {
   s.chars()
        .filter(|c| c.is_digit(10) || *c == ',')
        .collect::<String>()
        .replace(",", ".")
        .parse()
}

struct PageInfo {
    price: f32,
    title: String,
    // TODO: Availability, sizes
}

impl std::fmt::Debug for PageInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PageInfo")
            .field("price", &self.price)
            .field("title", &self.title)
            .finish()
    }
}

#[derive(Debug)]
enum PriceTrackerError{
    ExtractPageInfoError(String)
}

impl std::error::Error for PriceTrackerError {}

impl std::fmt::Display for PriceTrackerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExtractPageInfoError(inner) => write!(f, "Error extracting page info: {}", inner)
        }
    }
}


trait PageInfoExtractor {
    fn extract_page_info(&self, document: Document) -> Result<PageInfo, PriceTrackerError> ;
    fn match_url(&self, url: &str) -> bool;
}

struct ExtractorSkalnik {}

impl PageInfoExtractor for ExtractorSkalnik {
    fn extract_page_info(&self, document: Document) -> Result<PageInfo, PriceTrackerError> {
        let title = document.find(Name("title")).next().unwrap().text();
        let price_str = document.find(Class("price")).next().unwrap().text();
        match price_from_string(&price_str) {
            Ok(price) => Ok(PageInfo{title, price}),
            Err(e) => Err(PriceTrackerError::ExtractPageInfoError(e.to_string()))
        }
    }

    fn match_url(&self, url: &str) -> bool {
        return url.contains("8a.pl");
    }
}

struct Extractor8a {}

impl PageInfoExtractor for Extractor8a {
    fn extract_page_info(&self, document: Document) -> Result<PageInfo, PriceTrackerError> {
        let title = document.find(Name("title")).next().unwrap().text();
        let price_str = document
            .find(Descendant(Class("price-container"), Class("price")))
            .next()
            .unwrap()
            .text();

        match price_from_string(&price_str) {
            Ok(price) => Ok(PageInfo{title, price}),
            Err(e) => Err(PriceTrackerError::ExtractPageInfoError(e.to_string()))
        }
    }

    fn match_url(&self, url: &str) -> bool {
        return url.contains("skalnik.pl");
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let extractor_list: Vec<Box<dyn PageInfoExtractor>> = vec![
        Box::new(Extractor8a{}),
        Box::new(ExtractorSkalnik{}),
    ];

    let client = Client::new();
    let mut page_info_map: HashMap<String, PageInfo> = HashMap::new();

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

async fn get_page_info(client: &Client, url: &str, ext: &dyn PageInfoExtractor) -> Result<PageInfo, Box<dyn std::error::Error>> {
    let response_text = client.get(url).send().await?.text().await?;
    let document = Document::from(response_text.as_str());
    ext.extract_page_info(document).map_err(Into::into)
}
