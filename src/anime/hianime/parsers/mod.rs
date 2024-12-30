use reqwest::{
    header::{HeaderMap, HeaderValue, REFERER},
    Client,
};

use crate::util::new_http_client;

mod home_page;
mod search_suggestions;

const SRC_BASE_URL: &str = "https://hianime.to";
const SRC_HOME_URL: &str = "https://hianime.to/home";
const SRC_AJAX_URL: &str = "https://hianime.to/ajax";
const SRC_SEARCH_URL: &str = "https://hianime.to/search";

#[derive(Debug)]
pub struct Scraper {
    client: Client,
}

impl Scraper {
    pub fn new() -> Scraper {
        let mut headers = HeaderMap::new();
        headers.insert(REFERER, HeaderValue::from_static(SRC_BASE_URL));

        return Scraper {
            client: new_http_client(Some(headers)),
        };
    }
}
