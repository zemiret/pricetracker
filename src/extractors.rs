use std::num::ParseFloatError;

use select::{
    document::Document,
    predicate::{Class, Descendant, Name},
};

use crate::pricetracker;

pub trait PageInfoExtractor {
    fn extract_page_info(
        &self,
        document: Document,
    ) -> Result<pricetracker::PageInfo, pricetracker::Error>;
    fn match_url(&self, url: &str) -> bool;
    fn label(&self) -> &str;
}

pub struct Skalnik {}

impl PageInfoExtractor for Skalnik {
    fn extract_page_info(
        &self,
        document: Document,
    ) -> Result<pricetracker::PageInfo, pricetracker::Error> {
        let title = document.find(Name("title")).next().unwrap().text();
        let price_str = document
            .find(Descendant(Class("price-container"), Class("price")))
            .next()
            .ok_or(pricetracker::Error::ExtractPageInfoError(
                "price container not found".to_string(),
            ))?
            .text();

        match price_from_string(&price_str) {
            Ok(price) => Ok(pricetracker::PageInfo { title, price }),
            Err(e) => Err(pricetracker::Error::ExtractPageInfoError(e.to_string())),
        }
    }

    fn match_url(&self, url: &str) -> bool {
        return url.contains("skalnik.pl");
    }

    fn label(&self) -> &str {
        "Skalnik"
    }
}

pub struct A8a {}

impl PageInfoExtractor for A8a {
    fn extract_page_info(
        &self,
        document: Document,
    ) -> Result<pricetracker::PageInfo, pricetracker::Error> {
        let title = document.find(Name("title")).next().unwrap().text();
        let price_str = document
            .find(Class("price"))
            .next()
            .ok_or(pricetracker::Error::ExtractPageInfoError(
                "price container not found".to_string(),
            ))?
            .text();
        match price_from_string(&price_str) {
            Ok(price) => Ok(pricetracker::PageInfo { title, price }),
            Err(e) => Err(pricetracker::Error::ExtractPageInfoError(e.to_string())),
        }
    }

    fn match_url(&self, url: &str) -> bool {
        return url.contains("8a.pl");
    }

    fn label(&self) -> &str {
        "8a.pl"
    }
}

fn price_from_string(s: &str) -> Result<f32, ParseFloatError> {
    s.chars()
        .filter(|c| c.is_digit(10) || *c == ',')
        .collect::<String>()
        .replace(",", ".")
        .parse()
}
