use crate::{
    anime::hianime::{types::EpisodeInfo, utils::HiAnimeUtils, Scraper},
    utils::{EnmaClient, EnmaUtils},
    EnmaError, EnmaResult,
};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, REFERER},
    StatusCode,
};
use scraper::{Html, Selector};
use serde::Deserialize;

use super::ScrapedEpisodeServers;

// represents the raw json data we get from the source
#[derive(Deserialize, Debug)]
struct RawData {
    html: Option<String>,
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
    ///     match hianime.get_episode_servers(anime_episode_id).await {
    ///         Ok(data) => println!("{data:#?}"),
    ///         Err(e) => eprintln!("error: {e}"),
    ///     }
    /// }
    ///  ```
    pub async fn get_episode_servers(
        &self,
        anime_episode_id: &'static str,
    ) -> EnmaResult<ScrapedEpisodeServers> {
        const PROVIDER_PARSER: &'static str = "hianime:get_episode_servers";

        let episode_id: &'static str = anime_episode_id
            .trim()
            .split("?ep=")
            .last()
            .unwrap_or_default();
        if episode_id.is_empty() {
            return Err(EnmaError::misc_error(
                PROVIDER_PARSER,
                Some(String::from("invalid anime episode id")),
                Some(StatusCode::BAD_REQUEST),
            ));
        }

        let mut res = ScrapedEpisodeServers {
            anime_episode_id,
            ..Default::default()
        };

        let referer = format!("{}/watch/{anime_episode_id}", HiAnimeUtils::BaseUrl.value(),);
        let headers: HeaderMap = [
            (REFERER, HeaderValue::from_str(&referer).unwrap()),
            (
                HeaderName::from_static("x-requested-with"),
                HeaderValue::from_str(EnmaUtils::XRequestedWithHeader.value()).unwrap(),
            ),
        ]
        .into_iter()
        .collect();

        let url = format!(
            "{}?episodeId={episode_id}",
            HiAnimeUtils::EpisodeServersUrl.value()
        );

        let episodes_no_selector = &Selector::parse(".server-notice strong b").unwrap();
        let server_name_selector = &Selector::parse("a").unwrap();
        let server_selector = |name: &str| {
            return Selector::parse(&format!(
                ".ps_-block.ps_-block-sub.servers-{name} .ps__-list .server-item"
            ))
            .unwrap();
        };

        let data = self
            .client
            .get_json::<RawData>(url, Some(headers), PROVIDER_PARSER)
            .await?;

        let document = &Html::parse_document(&data.html.unwrap_or_default().as_str());

        ["sub", "dub", "raw"].map(|server| {
            for el in document.select(&server_selector(server)) {
                let server_id = el
                    .attr("data-server-id")
                    .map(|s| s.trim().parse::<u16>().ok())
                    .flatten();
                let server_name = el
                    .select(server_name_selector)
                    .next()
                    .and_then(|el| el.text().next())
                    .map(|s| s.to_lowercase().trim().to_string());

                let episode_info = EpisodeInfo {
                    server_id,
                    server_name,
                };

                match server {
                    "sub" => res.sub.push(episode_info),
                    "dub" => res.dub.push(episode_info),
                    "raw" => res.raw.push(episode_info),
                    _ => (),
                }
            }
        });

        res.episode_number = document
            .select(episodes_no_selector)
            .next()
            .and_then(|el| el.text().next())
            .and_then(|s| s.split(" ").last())
            .and_then(|s| s.trim().parse::<u16>().ok());

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::anime::hianime;
    use serde_json::to_string_pretty;

    // cargo test --lib -- anime::hianime::parsers::episode_servers::test --show-output
    #[tokio::test]
    async fn test_get_episode_servers() {
        let hianime = hianime::Scraper::new();
        let anime_episode_id = "steinsgate-0-92?ep=2055";

        match hianime.get_episode_servers(anime_episode_id).await {
            // Ok(_) => (),
            Ok(data) => {
                println!("{}", to_string_pretty(&data).unwrap());

                // assert_ne!(data.episodes.len(), 0);
                // assert_ne!(data.total_episodes, None);
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
