use scraper::{Html, Selector};

use super::types::{Anime, Episodes, MostPopularAnime, Top10Anime, TrendingAnime};

pub enum HiAnimeUtils {
    BaseUrl,
    HomeUrl,
    AjaxUrl,
    SearchUrl,
}

impl HiAnimeUtils {
    pub fn value(&self) -> &'static str {
        match self {
            HiAnimeUtils::BaseUrl => "https://hianime.to",
            HiAnimeUtils::HomeUrl => "https://hianime.to/home",
            HiAnimeUtils::AjaxUrl => "https://hianime.to/ajax",
            HiAnimeUtils::SearchUrl => "https://hianime.to/search",
        }
    }

    pub fn extract_trending_anime(document: &Html, selector: &Selector) -> Vec<TrendingAnime> {
        let mut anime: Vec<TrendingAnime> = Vec::with_capacity(10);

        let id_selector = &Selector::parse(".item .film-poster").unwrap();
        let name_selector = &Selector::parse(".item .number .film-title.dynamic-name").unwrap();
        let rank_selector = &Selector::parse(".item .number span").unwrap();
        let jname_selector = &Selector::parse(".item .number .film-title.dynamic-name").unwrap();
        let poster_selector = &Selector::parse(".item .film-poster .film-poster-img").unwrap();

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

    pub fn extract_top10_animes(document: &Html, period: &'static str) -> Vec<Top10Anime> {
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

    //
}
