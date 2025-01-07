use crate::{
    anime::hianime::{
        types::{Episodes, QtipAnime},
        utils::HiAnimeUtils,
        ScrapedQtipInfo, Scraper,
    },
    utils::{EnmaClient, EnmaUtils},
    EnmaError, EnmaResult,
};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, REFERER},
    StatusCode,
};
use scraper::{Html, Selector};

impl Scraper {
    pub async fn get_qtip_info(&self, anime_id: &'static str) -> EnmaResult<ScrapedQtipInfo> {
        const PROVIDER_PARSER: &'static str = "hianime:get_qtip_info";

        let anime_id = anime_id.trim();
        let id = anime_id.split('-').last().unwrap_or_default();

        if id.is_empty() {
            return Err(EnmaError::misc_error(
                PROVIDER_PARSER,
                Some(String::from("invalid anime id")),
                Some(StatusCode::BAD_REQUEST),
            ));
        }

        let url = format!("{}/{id}", HiAnimeUtils::QtipUrl.value());
        let headers: HeaderMap = [
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

        let page = self
            .client
            .get_html(url, Some(headers), PROVIDER_PARSER)
            .await?;
        let document = &Html::parse_document(&page);

        let id_selector = &Selector::parse(".pre-qtip-button a.btn-play").unwrap();
        let name_selector = &Selector::parse(".pre-qtip-title").unwrap();
        let mal_score_selector = &Selector::parse(".pre-qtip-detail .pqd-li").unwrap();
        let quality_selector = &Selector::parse(".tick .tick-quality").unwrap();
        let anime_type_selector = &Selector::parse(".badge.badge-quality").unwrap();
        let description_selector = &Selector::parse(".pre-qtip-description").unwrap();
        let sub_episodes_selector = &Selector::parse(".tick .tick-sub").unwrap();
        let dub_episodes_selector = &Selector::parse(".tick .tick-dub").unwrap();

        let other_info_selector = &Selector::parse(".pre-qtip-content .pre-qtip-line").unwrap();
        let sub_other_info_selector = &Selector::parse(".stick").unwrap();
        let genres_selector = &Selector::parse(".stick-text").unwrap();

        let id = document
            .select(id_selector)
            .next()
            .and_then(|el| el.value().attr("href"))
            .map(|s| s.split('/').last().map(|s| s.to_string()))
            .flatten();
        let name = document
            .select(name_selector)
            .next()
            .and_then(|el| el.text().next())
            .map(|s| s.trim().to_string());

        let anime_type = document
            .select(anime_type_selector)
            .next()
            .and_then(|el| el.text().next())
            .map(|s| s.trim().to_string());

        let mal_score = document
            .select(mal_score_selector)
            .next()
            .and_then(|el| el.text().next())
            .map(|s| s.trim().parse::<f64>().ok())
            .flatten();

        let description = document
            .select(description_selector)
            .next()
            .and_then(|el| el.text().next())
            .map(|s| s.trim().to_string());

        let quality = document
            .select(quality_selector)
            .next()
            .and_then(|el| el.text().next())
            .map(|s| s.trim().to_string());

        let sub = document
            .select(sub_episodes_selector)
            .next()
            .and_then(|el| el.text().next())
            .and_then(|s| s.trim().parse::<u16>().ok());

        let dub = document
            .select(dub_episodes_selector)
            .next()
            .and_then(|el| el.text().next())
            .and_then(|s| s.trim().parse::<u16>().ok());

        let mut jname = None;
        let mut synonyms = None;
        let mut aired = None;
        let mut status = None;
        let mut genres = vec![];

        for el in document.select(other_info_selector) {
            let key = el
                .select(sub_other_info_selector)
                .next() // Get the first element matching the selector
                .and_then(|el| el.text().next())
                .map(|s| s.trim().trim_end_matches(":").to_lowercase())
                .unwrap_or_default();

            let value = if key != "genres" {
                el.select(genres_selector)
                    .next()
                    .and_then(|el| el.text().next())
                    .map(|s| s.to_string())
                    .unwrap_or_default()
            } else {
                el.text().collect::<String>().trim()[key.len() + 1..]
                    .trim()
                    .to_string()
            };

            match key.as_str() {
                "japanese" => jname = Some(value),
                "synonyms" => synonyms = Some(value),
                "aired" => aired = Some(value),
                "status" => status = Some(value),
                "genres" => {
                    genres = value
                        .split(",") // from here
                        .map(|s| s.trim())
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                }
                _ => (),
            }
        }

        let res = ScrapedQtipInfo {
            anime: QtipAnime {
                id,
                name,
                description,
                mal_score,
                anime_type,
                quality,
                aired,
                genres,
                jname,
                status,
                synonyms,
                episodes: Episodes { sub, dub },
            },
        };
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::anime::hianime;
    use serde_json::to_string_pretty;

    // cargo test --lib -- anime::hianime::parsers::qtip::test --show-output
    #[tokio::test]
    async fn test_get_az_list() {
        let hianime = hianime::Scraper::new();
        let anime_id = "attack-on-titan-112";

        match hianime.get_qtip_info(anime_id).await {
            // Ok(_) => (),
            Ok(data) => println!("{}", to_string_pretty(&data).unwrap()),
            Err(e) => eprintln!("{}", e),
        }
    }
}
