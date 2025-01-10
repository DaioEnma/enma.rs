use crate::{
    anime::hianime::{
        parsers::types::ScrapedCategoryAnime, types::ANIME_CATEGORIES, utils::HiAnimeUtils, Scraper,
    },
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
    ///     let category = "most-favorite";
    ///     let page_number = Some(2);
    ///
    ///     match hianime.get_category_anime(category, page_number).await {
    ///         Ok(data) => println!("{data:#?}"),
    ///         Err(e) => eprintln!("error: {e}"),
    ///     }
    /// }
    ///  ```
    pub async fn get_category_anime(
        &self,
        category_name: &'static str,
        page_number: Option<u16>,
    ) -> EnmaResult<ScrapedCategoryAnime> {
        const PROVIDER_PARSER: &'static str = "hianime:get_category_anime";

        let category_name = category_name.trim();
        if !ANIME_CATEGORIES.contains(category_name) {
            return Err(EnmaError::misc_error(
                PROVIDER_PARSER,
                Some(String::from("invalid anime category")),
                Some(StatusCode::BAD_REQUEST),
            ));
        }

        let mut res = ScrapedCategoryAnime {
            current_page: page_number.unwrap_or(1).max(1),
            genres: Vec::with_capacity(41),
            ..Default::default()
        };

        let category_name_selector =
            &Selector::parse("#main-content .block_area .block_area-header .cat-heading").unwrap();
        let anime_selector =
            &Selector::parse("#main-content .tab-content .film_list-wrap .flw-item").unwrap();
        let genre_selector = &Selector::parse(
            "#main-sidebar .block_area.block_area_sidebar.block_area-genres .sb-genre-list li",
        )
        .unwrap();
        let top10_animes_selector =
            &Selector::parse("#main-sidebar .block_area-realtime [id^=\"top-viewed-\"]").unwrap();

        let url = format!(
            "{}/{category_name}?page={}",
            HiAnimeUtils::BaseUrl.value(),
            &res.current_page
        );

        let page = self.client.get_html(url, None, PROVIDER_PARSER).await?;
        let document = Html::parse_document(&page);

        res.category_name = document
            .select(category_name_selector)
            .next()
            .and_then(|e| e.text().next())
            .map(|s| s.to_string())
            .unwrap_or(category_name.to_string());
        res.total_pages = HiAnimeUtils::get_total_pages(&document);
        res.has_next_page = HiAnimeUtils::has_next_page(&document);
        res.animes = HiAnimeUtils::extract_animes(&document, anime_selector);
        res.top10_animes = HiAnimeUtils::extract_top10_animes(&document, top10_animes_selector);
        HiAnimeUtils::extract_genres(&document, genre_selector, &mut res.genres);

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::anime::hianime;
    use serde_json::to_string_pretty;

    // cargo test --lib -- anime::hianime::parsers::category::test --show-output
    #[tokio::test]
    async fn test_get_category_anime() {
        let hianime = hianime::Scraper::new();

        let category = "most-favorite";
        let page_number = Some(2);

        match hianime.get_category_anime(category, page_number).await {
            // Ok(_) => (),
            Ok(data) => {
                println!("{}", to_string_pretty(&data).unwrap());

                assert_eq!(data.total_pages, 187);
                assert_eq!(data.has_next_page, true);

                assert_ne!(data.animes.len(), 0);
                assert_ne!(data.genres.len(), 0);
                assert_ne!(data.top10_animes.today.len(), 0);
                assert_ne!(data.top10_animes.week.len(), 0);
                assert_ne!(data.top10_animes.month.len(), 0);
            }
            Err(e) => eprintln!("error {}", e),
        }
    }
}
