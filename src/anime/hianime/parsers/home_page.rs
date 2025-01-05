use crate::{
    anime::hianime::{parsers::types::ScrapedHomePage, utils::HiAnimeUtils, Scraper},
    error::EnmaResult,
    utils::EnmaClient,
};
use scraper::{Html, Selector};

impl Scraper {
    pub async fn get_home_page(&self) -> EnmaResult<ScrapedHomePage> {
        const PROVIDER_PARSER: &'static str = "hianime:get_home_page";
        let mut res = ScrapedHomePage {
            genres: Vec::with_capacity(41),
            ..Default::default()
        };

        let trending_selector =
            &Selector::parse("#trending-home .swiper-wrapper .swiper-slide").unwrap();
        let spotlight_selector = &Selector::parse("#slider .swiper-wrapper .swiper-slide").unwrap();
        let latest_episode_selector = &Selector::parse(
            "#main-content .block_area_home:nth-of-type(1) .tab-content .film_list-wrap .flw-item",
        )
        .unwrap();
        let top_upcoming_selector = &Selector::parse(
            "#main-content .block_area_home:nth-of-type(3) .tab-content .film_list-wrap .flw-item",
        )
        .unwrap();
        let genre_selector = &Selector::parse(
            "#main-sidebar .block_area.block_area_sidebar.block_area-genres .sb-genre-list li",
        )
        .unwrap();
        let most_viewed_selector =
            &Selector::parse("#main-sidebar .block_area-realtime [id^=\"top-viewed-\"]").unwrap();

        let top_airing_selector =
            &Selector::parse("#anime-featured .row div:nth-of-type(1) .anif-block-ul ul li")
                .unwrap();
        let most_popular_selector =
            &Selector::parse("#anime-featured .row div:nth-of-type(2) .anif-block-ul ul li")
                .unwrap();
        let most_favorite_selector =
            &Selector::parse("#anime-featured .row div:nth-of-type(3) .anif-block-ul ul li")
                .unwrap();
        let latest_completed_selector =
            &Selector::parse("#anime-featured .row div:nth-of-type(4) .anif-block-ul ul li")
                .unwrap();

        // raw html page
        let page = self
            .client
            .get_html(
                HiAnimeUtils::HomeUrl.value().to_string(),
                None,
                PROVIDER_PARSER,
            )
            .await?;

        let document = Html::parse_document(&page);

        res.spotlight_animes =
            HiAnimeUtils::extract_spotlight_animes(&document, spotlight_selector);

        res.trending_animes = HiAnimeUtils::extract_trending_anime(&document, trending_selector);
        res.latest_episode_animes =
            HiAnimeUtils::extract_animes(&document, latest_episode_selector);
        res.top_upcoming_animes = HiAnimeUtils::extract_animes(&document, top_upcoming_selector);

        // genres
        for el in document.select(genre_selector) {
            if let Some(genre) = el.text().next().map(|s| s.to_string()) {
                res.genres.push(genre);
            }
        }

        res.top10_animes = HiAnimeUtils::extract_top10_animes(&document, most_viewed_selector);

        res.top_airing_animes =
            HiAnimeUtils::extract_most_popular_anime(&document, top_airing_selector);
        res.most_popular_animes =
            HiAnimeUtils::extract_most_popular_anime(&document, most_popular_selector);
        res.most_favorite_animes =
            HiAnimeUtils::extract_most_popular_anime(&document, most_favorite_selector);
        res.latest_completed_animes =
            HiAnimeUtils::extract_most_popular_anime(&document, latest_completed_selector);

        return Ok(res);
    }
}

#[cfg(test)]
mod test {
    use crate::anime::hianime;
    use serde_json::to_string_pretty;

    // cargo test --lib -- anime::hianime::parsers::home_page::test --show-output
    #[tokio::test]
    async fn test_get_home_page() {
        let hianime = hianime::Scraper::new();
        match hianime.get_home_page().await {
            Ok(data) => println!("{}", to_string_pretty(&data).unwrap()),
            // Ok(_) => (),
            Err(e) => eprintln!("error {}", e),
        }
    }
}
