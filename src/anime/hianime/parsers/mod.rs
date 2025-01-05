use crate::{anime::hianime::utils::HiAnimeUtils, utils::EnmaUtils};
use reqwest::{
    header::{HeaderMap, HeaderValue, REFERER},
    Client,
};

mod types;
pub use types::*;

mod home_page;
mod search_suggestions;

#[derive(Debug)]
pub struct Scraper {
    client: Client,
}

impl Scraper {
    pub fn new() -> Self {
        let headers: HeaderMap = [(
            REFERER,
            HeaderValue::from_static(HiAnimeUtils::BaseUrl.value()),
        )]
        .into_iter()
        .collect();

        return Self {
            client: EnmaUtils::new_http_client(Some(headers)),
        };
    }
}
