use std::{collections::HashMap, process::Command};

use crate::{pricetracker, config::DEBUG_MODE};

pub struct Reqwest {
    client: reqwest::Client,
}

impl Reqwest {
    pub fn new(client: reqwest::Client) -> Reqwest {
        Reqwest { client }
    }
}

impl pricetracker::HttpClient for Reqwest {
    #[tokio::main]
    async fn get(&self, url: &str) -> Result<String, pricetracker::Error> {
        if DEBUG_MODE {
            println!("reqwest: get({})", url);
        }

        Ok(self
            .client
            .get(url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/116.0",
            )
            .send()
            .await
            .map_err(|x| pricetracker::Error::HttpGetError(format!("reqwest: send err {}", x)))?
            .text()
            .await
            .map_err(|x| pricetracker::Error::HttpGetError(format!("reqwest: text err {}", x)))?)
    }
}

pub struct Curl {}

impl Curl {
    pub fn new() -> Curl {
        Curl {}
    }
}

impl pricetracker::HttpClient for Curl {
    fn get(&self, url: &str) -> Result<String, pricetracker::Error> {
        if DEBUG_MODE {
            println!("curl: get({})", url);
        }

        let output = match Command::new("curl")
            .arg(url)
            .arg("--compressed")
            .arg("-H")
            .arg("User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/116.0")
            .output() {
                Err(e) => return Err(pricetracker::Error::HttpGetError(format!("curl: error executing curl {}: {}", url, e))),
                Ok(out) => out
            };

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let response_text = stdout.to_string();
            return Ok(response_text);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(pricetracker::Error::HttpGetError(format!(
                "curl: error getting {}: {}",
                url, stderr
            ))
            .into());
        }
    }
}

pub struct Mock {
    sitemap: HashMap<String, String>,
}

impl Mock {
    pub fn new(sitemap: HashMap<String, String>) -> Mock {
        Mock { sitemap }
    }
}

impl pricetracker::HttpClient for Mock {
    fn get(&self, url: &str) -> Result<String, pricetracker::Error> {
        if DEBUG_MODE {
            println!("mock: get({})", url);
        }

        Ok(self.sitemap.get(url).ok_or_else(||pricetracker::Error::HttpGetError(format!("mock: no entry for {}", url)))?.clone())
    }
}
