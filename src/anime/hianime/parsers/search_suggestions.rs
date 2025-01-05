use crate::{
    anime::hianime::{
        parsers::types::ScrapedSearchSuggestion, types::AnimeSearchSuggestion, utils::HiAnimeUtils,
        Scraper,
    },
    error::EnmaResult,
    utils::{EnmaClient, EnmaUtils},
};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, PRAGMA, REFERER};
use scraper::{Html, Selector};
use serde::Deserialize;

/// represents the raw data received from the source
#[derive(Deserialize, Debug)]
struct RawData {
    html: Option<String>,
}

impl Scraper {
    pub async fn get_search_suggestions(&self, q: &str) -> EnmaResult<ScrapedSearchSuggestion> {
        const PROVIDER_PARSER: &'static str = "hianime:get_search_suggestions";
        let mut res = ScrapedSearchSuggestion::default();

        let url = format!(
            "{}?keyword={}",
            HiAnimeUtils::SearchSuggestionUrl.value(),
            EnmaUtils::encode_uri_component(PROVIDER_PARSER, q.to_string())?
        );
        let headers: HeaderMap = [
            (ACCEPT, HeaderValue::from_static("*/*")),
            (PRAGMA, HeaderValue::from_static("no-cache")),
            (
                REFERER,
                HeaderValue::from_static(HiAnimeUtils::HomeUrl.value()),
            ),
            (
                HeaderName::from_static("x-requested-with"),
                HeaderValue::from_static(EnmaUtils::XRequestedWithHeader.value()),
            ),
        ]
        .into_iter()
        .collect();

        let selector = &Selector::parse(".nav-item:has(.film-poster)").unwrap();
        let name_selector = &Selector::parse(".srp-detail .film-name").unwrap();
        let jname_selector = &Selector::parse(".srp-detail .film-name").unwrap();
        let poster_selector = &Selector::parse(".film-poster .film-poster-img").unwrap();
        let other_info_selector = &Selector::parse(".film-infor").unwrap();

        let data = self
            .client
            .get_json::<RawData>(url, Some(headers), PROVIDER_PARSER)
            .await?;

        let document = Html::parse_fragment(data.html.unwrap_or_default().as_str());

        for el in document.select(selector) {
            let id = el
                .value()
                .attr("href")
                .map(|href| href[1..].split("?ref=search").next().map(|s| s.to_string()))
                .flatten();

            let name = el
                .select(name_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().to_string());

            let jname = el
                .select(jname_selector)
                .next()
                .and_then(|el| el.attr("data-jname"))
                .map(|s| s.trim().to_string());

            let poster = el
                .select(poster_selector)
                .next()
                .and_then(|el| el.attr("data-src"))
                .map(|s| s.trim().to_string());

            let other_info = el
                .select(other_info_selector)
                .next()
                .unwrap()
                .text()
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            res.suggestions.push(AnimeSearchSuggestion {
                id,
                name,
                jname,
                poster,
                other_info,
            });
        }

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::anime::hianime;
    use serde_json::to_string_pretty;

    // cargo test --lib -- anime::hianime::parsers::search_suggestions::test --show-output
    #[tokio::test]
    async fn test_search_suggestions() {
        let hianime = hianime::Scraper::new();
        let query = "monster";

        match hianime.get_search_suggestions(query).await {
            // Ok(_) => (),
            Ok(data) => println!("{}", to_string_pretty(&data).unwrap()),
            Err(e) => eprintln!("error {}", e),
        }
    }
}
