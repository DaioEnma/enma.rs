use crate::{
    anime::hianime::{
        parsers::types::ScrapedSearchResult, utils::HiAnimeUtils, Scraper, SearchFilters,
    },
    error::EnmaResult,
    utils::{EnmaClient, EnmaUtils},
    EnmaError,
};
use reqwest::{
    header::{HeaderMap, HeaderValue, REFERER},
    StatusCode,
};
use scraper::{Html, Selector};

impl Scraper {
    /// ### Example Usage
    /// ```rust
    /// async fn get_data() {
    ///     let hianime = hianime::Scraper::new();
    ///     let query = "monster";
    ///     let filters = SearchFilters::new(vec![
    ///         SearchFilter::Language(String::from("sub")),
    ///         SearchFilter::Sort(String::from("recently-added")),
    ///         SearchFilter::Genres(String::from("drama,comedy")),
    ///     ]);
    ///     let page_number = 1;
    ///
    ///     match hianime
    ///         .get_search_results(query, Some(page_number), Some(filters))
    ///         .await
    ///     {
    ///         Ok(data) => println!("{data:#?}"),
    ///         Err(e) => eprintln!("error {e}"),
    ///     }
    /// }
    ///  ```
    pub async fn get_search_results(
        &self,
        query: &'static str,
        page_number: Option<u16>,
        filters: Option<SearchFilters>,
    ) -> EnmaResult<ScrapedSearchResult> {
        const PROVIDER_PARSER: &'static str = "hianime:get_search_results";

        let query = query.trim();
        if query.is_empty() {
            return Err(EnmaError::invalid_data_error(
                PROVIDER_PARSER,
                Some(String::from("invalid search query")),
                Some(StatusCode::BAD_REQUEST),
            ));
        }

        let mut res = ScrapedSearchResult {
            search_query: query,
            current_page: page_number.unwrap_or(1).max(1),
            search_filter: filters.unwrap_or(SearchFilters::default()),
            ..Default::default()
        };

        let mut url = format!(
            "{}?keyword={}&page={}",
            HiAnimeUtils::SearchUrl.value(),
            EnmaUtils::decode_uri_component(PROVIDER_PARSER, query.to_string())?,
            res.current_page
        );
        if !res.search_filter.is_empty() {
            url = format!("{url}&{}", res.search_filter.to_query_params())
        }

        let headers: HeaderMap = [(
            REFERER,
            HeaderValue::from_static(HiAnimeUtils::HomeUrl.value()),
        )]
        .into_iter()
        .collect();

        let data = self
            .client
            .get_html(url, Some(headers), PROVIDER_PARSER)
            .await?;
        let document = Html::parse_fragment(&data);

        let animes_selector =
            &Selector::parse("#main-content .tab-content .film_list-wrap .flw-item").unwrap();
        let most_popular_selector = &Selector::parse(
            "#main-sidebar .block_area.block_area_sidebar.block_area-realtime .anif-block-ul ul li",
        )
        .unwrap();

        res.total_pages = HiAnimeUtils::get_total_pages(&document);
        res.has_next_page = HiAnimeUtils::has_next_page(&document);
        res.animes = HiAnimeUtils::extract_animes(&document, animes_selector);
        res.most_popular_animes =
            HiAnimeUtils::extract_most_popular_anime(&document, most_popular_selector);

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::anime::hianime::{self, SearchFilter, SearchFilters};
    use serde_json::to_string_pretty;

    // cargo test --lib -- anime::hianime::parsers::search::test --show-output
    #[tokio::test]
    async fn test_get_search_results() {
        let hianime = hianime::Scraper::new();
        let query = "monster";
        let filters = SearchFilters::new(vec![
            SearchFilter::Language(String::from("sub")),
            SearchFilter::Sort(String::from("recently-added")),
            SearchFilter::Genres(String::from("drama,comedy")),
        ]);
        let page_number = 1;

        match hianime
            .get_search_results(query, Some(page_number), Some(filters))
            .await
        {
            Ok(data) => {
                println!("{}", to_string_pretty(&data).unwrap());

                assert_ne!(data.animes.len(), 0);
            }
            Err(e) => eprintln!("error {}", e.details()),
        }
    }
}
