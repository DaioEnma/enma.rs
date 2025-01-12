use crate::{
    anime::hianime::{parsers::types::ScrapedGenreAnime, utils::HiAnimeUtils, Scraper},
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
    ///     let genre_name = "seinen";
    ///     let page_number = Some(2);
    ///
    ///     match hianime.get_genre_anime(genre_name, page_number).await {
    ///         Ok(data) => println!("{data:#?}"),
    ///         Err(e) => eprintln!("error: {e}"),
    ///     }
    /// }
    ///  ```
    pub async fn get_genre_anime(
        &self,
        genre_name: &'static str,
        page_number: Option<u16>,
    ) -> EnmaResult<ScrapedGenreAnime> {
        const PROVIDER_PARSER: &'static str = "hianime:get_genre_anime";

        let genre_name = genre_name.trim();
        if genre_name.is_empty() {
            return Err(EnmaError::invalid_data_error(
                PROVIDER_PARSER,
                Some(String::from("invalid genre name")),
                Some(StatusCode::BAD_REQUEST),
            ));
        }

        let mut res = ScrapedGenreAnime {
            genre_name: if genre_name == "martial-arts" {
                String::from("marial-arts")
            } else {
                genre_name.to_string()
            },
            genres: Vec::with_capacity(41),
            current_page: page_number.unwrap_or(1).max(1),
            ..Default::default()
        };

        let genre_name_selector =
            &Selector::parse("#main-content .block_area .block_area-header .cat-heading").unwrap();
        let genre_selector = &Selector::parse(
            "#main-sidebar .block_area.block_area_sidebar.block_area-genres .sb-genre-list li",
        )
        .unwrap();
        let anime_selector =
            &Selector::parse("#main-content .tab-content .film_list-wrap .flw-item").unwrap();
        let top_airing_selector =
            &Selector::parse("#main-sidebar .block_area_sidebar:nth-child(2) .block_area-content .anif-block-ul ul li")
                .unwrap();

        let url = format!(
            "{}/{genre_name}?page={}",
            HiAnimeUtils::GenreUrl.value(),
            &res.current_page
        );

        let page = self.client.get_html(url, None, PROVIDER_PARSER).await?;
        let document = Html::parse_document(&page);

        res.genre_name = document
            .select(genre_name_selector)
            .next()
            .and_then(|e| e.text().next())
            .map(|s| s.to_string())
            .unwrap_or(genre_name.to_string());

        res.total_pages = HiAnimeUtils::get_total_pages(&document);
        res.has_next_page = HiAnimeUtils::has_next_page(&document);

        HiAnimeUtils::extract_genres(&document, genre_selector, &mut res.genres);
        res.animes = HiAnimeUtils::extract_animes(&document, anime_selector);
        res.top_airing_animes =
            HiAnimeUtils::extract_most_popular_anime(&document, top_airing_selector);

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::anime::hianime;
    use serde_json::to_string_pretty;

    // cargo test --lib -- anime::hianime::parsers::genre::test --show-output
    #[tokio::test]
    async fn test_get_genre_anime() {
        let hianime = hianime::Scraper::new();

        let genre = "shounen";
        let page_number = Some(2);

        match hianime.get_genre_anime(genre, page_number).await {
            // Ok(_) => (),
            Ok(data) => {
                println!("{}", to_string_pretty(&data).unwrap());

                assert_eq!(data.total_pages, 43);
                assert_eq!(data.has_next_page, true);

                assert_ne!(data.animes.len(), 0);
                assert_ne!(data.top_airing_animes.len(), 0);
            }
            Err(e) => eprintln!("error {}", e),
        }
    }
}
