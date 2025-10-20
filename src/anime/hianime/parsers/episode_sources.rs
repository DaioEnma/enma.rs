use std::fmt::format;
use crate::{
    anime::{extractors::megacloud::Megacloud, hianime::{types::{Category, EpisodeInfo, Server}, utils::HiAnimeUtils, Scraper}},
    utils::{EnmaClient, EnmaUtils},
    EnmaError, EnmaResult,
};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, REFERER},
    StatusCode,
};
use scraper::{Html, Selector};
use serde::Deserialize;
use super::ScrapedEpisodeSources;

// represents the raw json data we get from the source
#[derive(Deserialize, Debug)]
struct RawData {
    html: Option<String>,
}

// represents the raw json data we get from the source
#[derive(Deserialize, Debug)]
struct SourcesRawData {
    link: Option<String>,
}

impl Scraper {
    /// ### Example Usage
    /// ```rust
    /// async fn get_data() {
    ///     use enma::anime::hianime;
    ///     let hianime = hianime::Scraper::new();
    ///
    ///     let anime_episode_id = "steinsgate-0-92?ep=2055";
    ///
    ///     match hianime.get_episode_sources(anime_episode_id).await {
    ///         Ok(data) => println!("{data:#?}"),
    ///         Err(e) => eprintln!("error: {e}"),
    ///     }
    /// }
    ///  ```
    pub async fn get_episode_sources(
        &self,
        anime_episode_id: &'static str,
        server: Server,
        category: Category,
    ) -> EnmaResult<ScrapedEpisodeSources> {
        const PROVIDER_PARSER: &'static str = "hianime:get_episode_sources";

        let episode_id: &'static str = anime_episode_id
            .trim()
            .split("?ep=")
            .last()
            .unwrap_or_default();
        let url = format!(
            "{}/v2/episode/servers?episodeId={episode_id}",
            HiAnimeUtils::AjaxUrl.value()
        );
        let referer = format!(
            "{}/watch/{}",
            HiAnimeUtils::BaseUrl.value(),
            anime_episode_id
        );

        let headers: HeaderMap = [
            (REFERER, HeaderValue::from_str(&referer).unwrap()),
            (
                HeaderName::from_static("x-requested-with"),
                HeaderValue::from_str(EnmaUtils::XRequestedWithHeader.value()).unwrap(),
            ),
        ]
            .into_iter()
            .collect();

        let data = self
            .client
            .get_json::<RawData>(url, Some(headers.clone()), PROVIDER_PARSER)
            .await?;

        let document = Html::parse_document(&data.html.unwrap_or_default().as_str());

        let server_id = match retrieve_server_id(&document, server.index(), category.value()) {
            Some(id) => id,
            None => {
                return Err(EnmaError::src_fetch_error(
                    PROVIDER_PARSER,
                    Some(String::from("server id not found")),
                    Some(StatusCode::NOT_FOUND),
                ));
            }
        };

        let url = format!(
            "{}/v2/episode/sources?id={server_id}",
            HiAnimeUtils::AjaxUrl.value()
        );

        let data = self
            .client
            .get_json::<SourcesRawData>(url, Some(headers), PROVIDER_PARSER)
            .await?;

        let embed_link = match data.link {
            Some(link) => link,
            None => {
                return Err(EnmaError::src_fetch_error(
                    PROVIDER_PARSER,
                    Some(String::from("embed link not found")),
                    Some(StatusCode::NOT_FOUND),
                ));
            }
        };

        let origin = embed_link
            .trim().split("/").nth(2)
            .unwrap_or_default();
        match server {
            Server::VidStreaming | Server::VidCloud => {
                let megacloud = Megacloud::new(self.client.clone());
                let extracted = megacloud.extract(&embed_link).await.unwrap();
                return Ok(ScrapedEpisodeSources {
                    headers: format!("https://{}", origin),
                    extracted,
                });
            },
            _ => {
                return Err(EnmaError::src_fetch_error(
                    PROVIDER_PARSER,
                    Some(String::from("only VidStreaming and VidCloud servers are supported for full URLs")),
                    Some(StatusCode::BAD_REQUEST),
                ));
            }
        }
    }
}

fn retrieve_server_id(
    document: &Html,
    index: u16,
    category: &'static str,
) -> Option<String> {
    let selector_str = format!(
        ".ps_-block.ps_-block-sub.servers-{} > .ps__-list .server-item",
        category
    );
    let selector = Selector::parse(&selector_str).ok()?;

    for element in document.select(&selector) {
        if let Some(server_id) = element.value().attr("data-server-id") {
            if server_id == index.to_string() {
                return element.value().attr("data-id").map(|s| s.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use crate::anime::hianime::{self, types::{Category, Server}};
    use serde_json::to_string_pretty;

    // cargo test --lib -- anime::hianime::parsers::episode_sources::test --show-output
    #[tokio::test]
    async fn test_get_episode_sources() {
        let hianime = hianime::Scraper::new();
        let anime_episode_id = "steinsgate-0-92?ep=2055";

        match hianime.get_episode_sources(anime_episode_id, Server::VidStreaming, Category::Sub).await {
            Ok(data) => {
                println!("{}", to_string_pretty(&data).unwrap());

                assert_ne!(data.headers, "");
                assert_ne!(data.extracted.subtitles.len(), 0);
                assert_ne!(data.extracted.sources.len(), 0);
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
