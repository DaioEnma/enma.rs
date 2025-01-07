#![allow(dead_code)] // TODO: remove this

use super::{
    types::{
        Anime, Episodes, MostPopularAnime, SpotlightAnime, Top10Anime, Top10AnimePeriod,
        Top10AnimesWithPeriod, TrendingAnime,
    },
    SEARCH_PAGE_FILTERS,
};
use scraper::{Html, Selector};

pub enum HiAnimeUtils {
    BaseUrl,
    HomeUrl,
    AjaxUrl,
    SearchUrl,
    SearchSuggestionUrl,
    AZListUrl,
    QtipUrl,
    ProducerUrl,
}

impl HiAnimeUtils {
    pub fn value(&self) -> &'static str {
        match self {
            HiAnimeUtils::BaseUrl => "https://hianime.to",
            HiAnimeUtils::HomeUrl => "https://hianime.to/home",
            HiAnimeUtils::AjaxUrl => "https://hianime.to/ajax",
            HiAnimeUtils::SearchUrl => "https://hianime.to/search",
            HiAnimeUtils::SearchSuggestionUrl => "https://hianime.to/ajax/search/suggest",
            HiAnimeUtils::AZListUrl => "https://hianime.to/az-list",
            HiAnimeUtils::QtipUrl => "https://hianime.to/ajax/movie/qtip",
            HiAnimeUtils::ProducerUrl => "https://hianime.to/producer",
        }
    }

    pub fn extract_trending_anime(document: &Html, selector: &Selector) -> Vec<TrendingAnime> {
        let mut anime: Vec<TrendingAnime> = Vec::with_capacity(10);

        let id_selector = &Selector::parse(".item .film-poster").unwrap();
        let name_selector = &Selector::parse(".item .number .film-title.dynamic-name").unwrap();
        let jname_selector = name_selector;
        let poster_selector = &Selector::parse(".item .film-poster .film-poster-img").unwrap();
        let rank_selector = &Selector::parse(".item .number span").unwrap();

        for el in document.select(selector) {
            let id = el
                .select(id_selector)
                .next()
                .and_then(|el| el.value().attr("href"))
                .map(|s| s[1..].trim().to_string());

            let name = el
                .select(name_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().to_string());

            let jname = el
                .select(jname_selector)
                .next()
                .and_then(|el| el.value().attr("data-jname"))
                .map(|s| s.trim().to_string());

            let rank = el
                .select(rank_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().parse::<u32>().ok())
                .flatten();

            let poster = el
                .select(poster_selector)
                .next()
                .and_then(|el| el.value().attr("data-src"))
                .map(|s| s.trim().to_string());

            anime.push(TrendingAnime {
                id,
                rank,
                name,
                jname,
                poster,
            });
        }

        return anime;
    }

    pub fn extract_animes(document: &Html, selector: &Selector) -> Vec<Anime> {
        let mut anime = vec![];

        let id_selector = &Selector::parse(".film-detail .film-name .dynamic-name").unwrap();
        let name_selector = id_selector;
        let jname_selector = id_selector;
        let poster_selector = &Selector::parse(".film-poster .film-poster-img").unwrap();
        let duration_selector =
            &Selector::parse(".film-detail .fd-infor .fdi-item.fdi-duration").unwrap();
        let anime_type_selector =
            &Selector::parse(".film-detail .fd-infor .fdi-item:nth-of-type(1)").unwrap();
        let rating_selector = &Selector::parse(".film-poster .tick-rate").unwrap();
        let sub_episodes_selector = &Selector::parse(".film-poster .tick-sub").unwrap();
        let dub_episodes_selector = &Selector::parse(".film-poster .tick-dub").unwrap();

        for el in document.select(selector) {
            let id = el
                .select(id_selector)
                .next()
                .and_then(|el| el.attr("href"))
                .map(|s| {
                    s[1..]
                        .split_once("?ref=search")
                        .map_or_else(|| s[1..].to_string(), |(part, _)| part.to_string())
                });

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

            let duration = el
                .select(duration_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().to_string());

            let anime_type = el
                .select(anime_type_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().to_string());

            let rating = el
                .select(rating_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().to_string());

            let dub = el
                .select(dub_episodes_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.split_whitespace().last())
                .flatten()
                .and_then(|s| s.parse::<u16>().ok());

            let sub = el
                .select(sub_episodes_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.split_whitespace().last())
                .flatten()
                .and_then(|s| s.parse::<u16>().ok());

            anime.push(Anime {
                id,
                name,
                jname,
                poster,
                duration,
                anime_type,
                rating,
                episodes: Episodes { sub, dub },
            });
        }

        return anime;
    }

    pub fn extract_most_popular_anime(
        document: &Html,
        selector: &Selector,
    ) -> Vec<MostPopularAnime> {
        let mut anime = vec![];

        let id_selector = &Selector::parse(".film-detail .film-name .dynamic-name").unwrap();
        let name_selector = id_selector;
        let jname_selector = id_selector;
        let poster_selector = &Selector::parse(".film-poster .film-poster-img").unwrap();
        let anime_type_selector = &Selector::parse(".fd-infor .tick .fdi-item").unwrap();
        let sub_episodes_selector = &Selector::parse(".fd-infor .tick .tick-sub").unwrap();
        let dub_episodes_selector = &Selector::parse(".fd-infor .tick .tick-dub").unwrap();

        for el in document.select(selector) {
            let id = el
                .select(id_selector)
                .next()
                .and_then(|el| el.value().attr("href"))
                .map(|s| s[1..].trim().to_string());

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

            let anime_type = el
                .select(anime_type_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().to_string());

            let dub = el
                .select(dub_episodes_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.split_whitespace().last())
                .flatten()
                .and_then(|s| s.parse::<u16>().ok());

            let sub = el
                .select(sub_episodes_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.split_whitespace().last())
                .flatten()
                .and_then(|s| s.parse::<u16>().ok());

            anime.push(MostPopularAnime {
                id,
                name,
                jname,
                poster,
                anime_type,
                episodes: Episodes { sub, dub },
            });
        }

        return anime;
    }

    fn extract_top10_anime(document: &Html, period: &String) -> Vec<Top10Anime> {
        let selector = &Selector::parse(format!("#top-viewed-{} ul li", period).as_str()).unwrap();
        let mut anime = Vec::with_capacity(10);

        let id_selector = &Selector::parse(".film-detail .film-name .dynamic-name").unwrap();
        let name_selector = id_selector;
        let jname_selector = id_selector;
        let rank_selector = &Selector::parse(".film-number span").unwrap();
        let poster_selector = &Selector::parse(".film-poster .film-poster-img").unwrap();
        let sub_episodes_selector = &Selector::parse(".fd-infor .tick .tick-sub").unwrap();
        let dub_episodes_selector = &Selector::parse(".fd-infor .tick .tick-dub").unwrap();

        for el in document.select(selector) {
            let id = el
                .select(id_selector)
                .next()
                .and_then(|el| el.value().attr("href"))
                .map(|s| s[1..].trim().to_string());

            let name = el
                .select(name_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().to_string());

            let jname = el
                .select(jname_selector)
                .next()
                .and_then(|el| el.value().attr("data-jname"))
                .map(|s| s.trim().to_string());

            let rank = el
                .select(rank_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().parse::<u32>().ok())
                .flatten();

            let poster = el
                .select(poster_selector)
                .next()
                .and_then(|el| el.value().attr("data-src"))
                .map(|s| s.trim().to_string());

            let sub = el
                .select(sub_episodes_selector)
                .next()
                .and_then(|el| el.text().next())
                .and_then(|s| s.trim().parse::<u16>().ok());

            let dub = el
                .select(dub_episodes_selector)
                .next()
                .and_then(|el| el.text().next())
                .and_then(|s| s.trim().parse::<u16>().ok());

            anime.push(Top10Anime {
                id,
                name,
                poster,
                jname,
                rank,
                episodes: Episodes { sub, dub },
            });
        }

        return anime;
    }

    pub fn extract_top10_animes(document: &Html, selector: &Selector) -> Top10AnimesWithPeriod {
        let mut top10_animes = Top10AnimesWithPeriod::default();

        for el in document.select(selector) {
            let period = el
                .attr("id")
                .map(|s| s.split("-").last().map(|s| s.to_string()))
                .flatten();

            if let Some(time_period) = period {
                match Top10AnimePeriod::from(&time_period) {
                    Top10AnimePeriod::Day => {
                        top10_animes.today = Self::extract_top10_anime(&document, &time_period)
                    }
                    Top10AnimePeriod::Week => {
                        top10_animes.week = Self::extract_top10_anime(&document, &time_period)
                    }
                    Top10AnimePeriod::Month => {
                        top10_animes.month = Self::extract_top10_anime(&document, &time_period)
                    }
                }
            }
        }

        return top10_animes;
    }

    pub fn extract_spotlight_animes(document: &Html, selector: &Selector) -> Vec<SpotlightAnime> {
        let mut anime = Vec::with_capacity(10);

        let id_selector = &Selector::parse(".deslide-item-content .desi-buttons a").unwrap();
        let name_selector =
            &Selector::parse(".deslide-item-content .desi-head-title.dynamic-name").unwrap();
        let jname_selector = name_selector;
        let poster_selector =
            &Selector::parse(".deslide-cover .deslide-cover-img .film-poster-img").unwrap();
        let rank_selector = &Selector::parse(".deslide-item-content .desi-sub-text").unwrap();
        let description_selector =
            &Selector::parse(".deslide-item-content .desi-description").unwrap();
        let sub_episodes_selector =
            &Selector::parse(".deslide-item-content .sc-detail .scd-item .tick-item.tick-sub")
                .unwrap();
        let dub_episodes_selector =
            &Selector::parse(".deslide-item-content .sc-detail .scd-item .tick-item.tick-dub")
                .unwrap();
        let other_info_selector =
            &Selector::parse(".deslide-item-content .sc-detail .scd-item").unwrap();

        for el in document.select(selector) {
            let id = el
                .select(id_selector)
                .last()
                .and_then(|el| el.value().attr("href"))
                .map(|s| s[1..].trim().to_string());

            let name = el
                .select(name_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().to_string());

            let jname = el
                .select(jname_selector)
                .next()
                .and_then(|el| el.value().attr("data-jname"))
                .map(|s| s.trim().to_string());

            let poster = el
                .select(poster_selector)
                .next()
                .and_then(|el| el.value().attr("data-src"))
                .map(|s| s.trim().to_string());

            let description = el
                .select(description_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().to_string());

            let rank = el
                .select(rank_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.split_whitespace().next())
                .flatten()
                .and_then(|s| s[1..].parse::<u32>().ok());

            let sub = el
                .select(sub_episodes_selector)
                .next()
                .and_then(|el| el.text().next())
                .and_then(|s| s.trim().parse::<u16>().ok());

            let dub = el
                .select(dub_episodes_selector)
                .next()
                .and_then(|el| el.text().next())
                .and_then(|s| s.trim().parse::<u16>().ok());

            let mut other_info = el
                .select(other_info_selector)
                .filter_map(|el| el.text().next())
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();

            other_info = if other_info.is_empty() {
                other_info
            } else {
                other_info[0..other_info.len() - 1].to_vec()
            };

            let anime_type = other_info.first().map(|s| s.to_owned());

            anime.push(SpotlightAnime {
                id,
                name,
                jname,
                poster,
                description,
                rank,
                episodes: Episodes { sub, dub },
                anime_type,
                other_info,
            });
        }

        return anime;
    }

    pub fn extract_genres(
        document: &Html,
        genre_selector: &Selector,
        genre_names: &mut Vec<String>,
    ) {
        for el in document.select(genre_selector) {
            if let Some(genre) = el.text().next().map(|s| s.to_string()) {
                genre_names.push(genre);
            }
        }
    }

    pub fn has_next_page(document: &Html) -> bool {
        let pagination_selector = &Selector::parse(".pagination > li").unwrap();
        let active_selector = &Selector::parse(".pagination li.active").unwrap();

        if document.select(pagination_selector).count() == 0 {
            return false;
        }
        if document.select(active_selector).count() == 0 {
            return false;
        }

        if let Some(last_li) = document.select(pagination_selector).last() {
            return !last_li.value().classes().any(|class| class == "active");
        }
        false
    }

    pub fn get_total_pages(document: &Html) -> u16 {
        let pagination_last_selector =
            Selector::parse(r#".pagination > .page-item a[title="Last"]"#).unwrap();
        let pagination_next_selector =
            Selector::parse(r#".pagination > .page-item a[title="Next"]"#).unwrap();
        let pagination_active_selector =
            Selector::parse(".pagination > .page-item.active a").unwrap();

        let last_page = document
            .select(&pagination_last_selector)
            .filter_map(|el| el.value().attr("href"))
            .flat_map(|href| href.split('=').last())
            .next();

        let next_page = document
            .select(&pagination_next_selector)
            .filter_map(|el| el.value().attr("href"))
            .flat_map(|href| href.split('=').last())
            .next();

        let active_page = document
            .select(&pagination_active_selector)
            .flat_map(|el| el.text().collect::<Vec<_>>())
            .next()
            .map(|s| s.parse::<u16>().ok())
            .flatten();

        let total_pages = last_page
            .or(next_page)
            .and_then(|page| page.parse::<u16>().ok())
            .or(active_page)
            .unwrap_or(1);

        total_pages
    }

    pub fn get_genres_filter_value(genre_names: Vec<String>) -> Option<String> {
        if genre_names.is_empty() {
            return None;
        };

        let value = genre_names
            .iter()
            .filter_map(|name| {
                SEARCH_PAGE_FILTERS
                    .genres_id_map
                    .get(name.as_str())
                    .map(|s| s.to_string())
            })
            .collect::<Vec<String>>()
            .join(",");

        Some(value)
    }
}
