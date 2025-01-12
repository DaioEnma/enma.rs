use crate::{
    anime::hianime::{parsers::types::ScrapedProducerAnime, utils::HiAnimeUtils, Scraper},
    error::EnmaResult,
    utils::EnmaClient,
    EnmaError,
};
use reqwest::StatusCode;
use scraper::{Html, Selector};

impl Scraper {
    /// ### Example Usage
    /// ```rust
    /// async fn get_data() {
    ///     use enma::anime::hianime;
    ///     let hianime = hianime::Scraper::new();
    ///
    ///     let producer_name = "toei-animation";
    ///     let page_number = Some(2);
    ///
    ///     match hianime.get_producer_anime(producer_name, page_number).await {
    ///         Ok(data) => println!("{data:#?}"),
    ///         Err(e) => eprintln!("error: {e}"),
    ///     }
    /// }
    ///  ```
    pub async fn get_producer_anime(
        &self,
        producer_name: &'static str,
        page_number: Option<u16>,
    ) -> EnmaResult<ScrapedProducerAnime> {
        const PROVIDER_PARSER: &'static str = "hianime:get_producer_anime";

        let producer_name = producer_name.trim();
        if producer_name.is_empty() {
            return Err(EnmaError::invalid_data_error(
                PROVIDER_PARSER,
                Some(String::from("invalid producer name")),
                Some(StatusCode::BAD_REQUEST),
            ));
        }

        let mut res = ScrapedProducerAnime {
            current_page: page_number.unwrap_or(1).max(1),
            ..Default::default()
        };

        let producer_name_selector =
            &Selector::parse("#main-content .block_area .block_area-header .cat-heading").unwrap();
        let anime_selector =
            &Selector::parse("#main-content .tab-content .film_list-wrap .flw-item").unwrap();
        let top10_animes_selector =
            &Selector::parse(r#"#main-sidebar .block_area-realtime [id^="top-viewed-"]"#).unwrap();
        let top_airing_selector =
            &Selector::parse("#main-sidebar .block_area_sidebar:nth-child(2) .block_area-content .anif-block-ul ul li")
                .unwrap();

        let url = format!(
            "{}/{producer_name}?page={}",
            HiAnimeUtils::ProducerUrl.value(),
            &res.current_page
        );

        let page = self.client.get_html(url, None, PROVIDER_PARSER).await?;
        let document = Html::parse_document(&page);

        res.producer_name = document
            .select(producer_name_selector)
            .next()
            .and_then(|e| e.text().next())
            .map(|s| s.to_string())
            .unwrap_or(producer_name.to_string());
        res.total_pages = HiAnimeUtils::get_total_pages(&document);
        res.has_next_page = HiAnimeUtils::has_next_page(&document);
        res.animes = HiAnimeUtils::extract_animes(&document, anime_selector);
        res.top10_animes = HiAnimeUtils::extract_top10_animes(&document, top10_animes_selector);
        res.top_airing_animes =
            HiAnimeUtils::extract_most_popular_anime(&document, top_airing_selector);

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::anime::hianime;
    use serde_json::to_string_pretty;

    // cargo test --lib -- anime::hianime::parsers::producer::test --show-output
    #[tokio::test]
    async fn test_get_producer_animee() {
        let hianime = hianime::Scraper::new();

        let producer = "toei-animation";
        let page_number = Some(2);

        match hianime.get_producer_anime(producer, page_number).await {
            // Ok(_) => (),
            Ok(data) => {
                println!("{}", to_string_pretty(&data).unwrap());

                assert_eq!(data.total_pages, 12);
                assert_eq!(data.has_next_page, true);

                assert_ne!(data.animes.len(), 0);
                assert_ne!(data.top_airing_animes.len(), 0);
                assert_ne!(data.top10_animes.today.len(), 0);
                assert_ne!(data.top10_animes.week.len(), 0);
                assert_ne!(data.top10_animes.month.len(), 0);
            }
            Err(e) => eprintln!("error {}", e),
        }
    }
}
