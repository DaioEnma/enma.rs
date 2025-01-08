use crate::{
    anime::hianime::{types::AnimeEpisode, utils::HiAnimeUtils, ScrapedAnimeEpisodes, Scraper},
    utils::{EnmaClient, EnmaUtils},
    EnmaError, EnmaResult,
};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, REFERER},
    StatusCode,
};
use scraper::{Html, Selector};
use serde::Deserialize;

// represents the raw json data we get from the source
#[derive(Deserialize, Debug)]
struct RawData {
    html: Option<String>,
}

impl Scraper {
    pub async fn get_anime_episodes(
        &self,
        anime_id: &'static str,
    ) -> EnmaResult<ScrapedAnimeEpisodes> {
        const PROVIDER_PARSER: &'static str = "hianime:get_anime_episodes";

        let anime_id: &'static str = anime_id.trim().split('-').last().unwrap_or_default();
        if anime_id.is_empty() {
            return Err(EnmaError::misc_error(
                PROVIDER_PARSER,
                Some(String::from("invalid anime id")),
                Some(StatusCode::BAD_REQUEST),
            ));
        }

        let referer = format!("{}/watch/{anime_id}", HiAnimeUtils::BaseUrl.value());
        let headers: HeaderMap = [
            (REFERER, HeaderValue::from_str(&referer).unwrap()),
            (
                HeaderName::from_static("x-requested-with"),
                HeaderValue::from_str(EnmaUtils::XRequestedWithHeader.value()).unwrap(),
            ),
        ]
        .into_iter()
        .collect();
        let url = format!("{}/{anime_id}", HiAnimeUtils::EpisodeListUrl.value());

        let data = self
            .client
            .get_json::<RawData>(url, Some(headers), PROVIDER_PARSER)
            .await?;

        let document = &Html::parse_document(&data.html.unwrap_or_default().as_str());
        let episodes_selector = &Selector::parse(".detail-infor-content .ss-list a").unwrap();

        let mut res = ScrapedAnimeEpisodes::default();

        res.total_episodes = match u16::try_from(document.select(episodes_selector).count()) {
            Ok(val) => Some(val),
            Err(_) => None,
        };

        for el in document.select(episodes_selector) {
            let title = el.attr("title").map(|s| s.trim().to_string());

            let number = el
                .attr("data-number")
                .map(|s| s.trim().parse::<u16>().ok())
                .flatten();

            let is_filler = el.value().classes().any(|class| class == "ssl-item-filler");

            let episode_id = el
                .attr("href")
                .map(|s| s.split("/").last().map(|s| s.to_string()))
                .flatten();

            res.episodes.push(AnimeEpisode {
                title,
                number,
                is_filler,
                episode_id,
            });
        }

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::anime::hianime;
    use serde_json::to_string_pretty;

    // cargo test --lib -- anime::hianime::parsers::episodes::test --show-output
    #[tokio::test]
    async fn test_get_anime_episodes() {
        let hianime = hianime::Scraper::new();
        let anime_id = "steinsgate-3";

        match hianime.get_anime_episodes(anime_id).await {
            // Ok(_) => (),
            Ok(data) => {
                println!("{}", to_string_pretty(&data).unwrap());

                assert_ne!(data.episodes.len(), 0);
                assert_ne!(data.total_episodes, None);
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
