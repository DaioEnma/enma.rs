use crate::{anime::hianime::utils::HiAnimeUtils, utils::EnmaUtils};
use reqwest::{
    header::{HeaderMap, HeaderValue, REFERER},
    Client,
};

mod types;
pub use types::*;

mod az_list;
mod category;
mod episode_servers;
mod episodes;
mod genre;
mod home_page;
mod info;
mod producer;
mod qtip;
mod schedule;
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
