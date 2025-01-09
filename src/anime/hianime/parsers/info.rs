use crate::{
    anime::hianime::{
        parsers::ScrapedAnimeInfo,
        types::{
            AnimeCharacter, AnimeCharactersVoiceActors, AnimePromotionalVideo, OtherInfoValue,
            Season,
        },
        utils::HiAnimeUtils,
        Scraper,
    },
    error::{EnmaError, EnmaResult},
    utils::EnmaClient,
};
use regex::Regex;
use reqwest::StatusCode;
use scraper::{Html, Selector};

impl Scraper {
    pub async fn get_info(&self, anime_id: &'static str) -> EnmaResult<ScrapedAnimeInfo> {
        const PROVIDER_PARSER: &'static str = "hianime:get_info";

        let anime_id = anime_id.trim();
        if anime_id.is_empty() || !anime_id.contains('-') {
            return Err(EnmaError::misc_error(
                PROVIDER_PARSER,
                Some(String::from("invalid anime id")),
                Some(StatusCode::BAD_REQUEST),
            ));
        }

        let mut res = ScrapedAnimeInfo::default();

        let url = format!("{}/{anime_id}", HiAnimeUtils::BaseUrl.value());
        let page = self.client.get_html(url, None, PROVIDER_PARSER).await?;

        let document = Html::parse_document(&page);

        let id_selector = &Selector::parse(".anisc-detail .film-buttons a.btn-play").unwrap();
        let name_selector = &Selector::parse(".anisc-detail .film-name.dynamic-name").unwrap();
        let jname_selector = name_selector;
        let description_selector =
            &Selector::parse(".anisc-detail .film-description .text").unwrap();
        let poster_selector = &Selector::parse(".film-poster .film-poster-img").unwrap();

        let seasons_selector = &Selector::parse("#main-content .os-list a.os-item").unwrap();
        let season_title_selector = &Selector::parse(".title").unwrap();
        let season_poster_selector = &Selector::parse(".season-poster").unwrap();

        let rating_selector = &Selector::parse(".anis-content .film-stats .tick .tick-pg").unwrap();
        let quality_selector =
            &Selector::parse(".anis-content .film-stats .tick .tick-quality").unwrap();
        let sub_episodes_selector =
            &Selector::parse(".anis-content .film-stats .tick .tick-sub").unwrap();
        let dub_episodes_selector =
            &Selector::parse(".anis-content .film-stats .tick .tick-dub").unwrap();
        let other_stats_selector = &Selector::parse(".anis-content .film-stats .tick").unwrap();

        let other_info_selector =
            &Selector::parse(".anisc-info-wrap .anisc-info .item:not(.w-hide)").unwrap();
        let other_info_key_selector = &Selector::parse(".item-head").unwrap();
        let other_info_value_selector = &Selector::parse("*:not(.item-head)").unwrap();

        let promotional_videos_selector = &Selector::parse(
            ".block_area.block_area-promotions .block_area-promotions-list .screen-items .item",
        )
        .unwrap();
        let promotional_video_thumbnail_selector = &Selector::parse("img").unwrap();

        let char_vc_actor_selector = &Selector::parse(
            ".block_area.block_area-actors .block-actors-content .bac-list-wrap .bac-item",
        )
        .unwrap();
        let char_id_selector = &Selector::parse(".per-info.ltr .pi-avatar").unwrap();
        let char_poster_selector = &Selector::parse(".per-info.ltr .pi-avatar img").unwrap();
        let char_name_selector = &Selector::parse(".per-info.ltr .pi-detail a").unwrap();
        let char_cast_selector = &Selector::parse(".per-info.ltr .pi-detail .pi-cast").unwrap();

        let vc_actor_id_selector = &Selector::parse(".per-info.rtl .pi-avatar").unwrap();
        let vc_actor_name_selector = &Selector::parse(".per-info.rtl .pi-detail a").unwrap();
        let vc_actor_poster_selector = &Selector::parse(".per-info.rtl .pi-avatar img").unwrap();
        let vc_actor_cast_selector = &Selector::parse(".per-info.rtl .pi-detail .pi-cast").unwrap();

        let related_anime_selector = &Selector::parse("#main-sidebar .block_area.block_area_sidebar.block_area-realtime:nth-of-type(1) .anif-block-ul ul li").unwrap();
        let most_popular_selector = &Selector::parse("#main-sidebar .block_area.block_area_sidebar.block_area-realtime:nth-of-type(2) .anif-block-ul ul li").unwrap();
        let recommended_anime_selector = &Selector::parse(
            "#main-content .block_area.block_area_category .tab-content .flw-item",
        )
        .unwrap();

        // anime info
        {
            res.anime.id = document
                .select(id_selector)
                .next()
                .and_then(|el| el.attr("href"))
                .and_then(|s| s.split('/').last())
                .map(|s| s.to_string());
            res.anime.name = document
                .select(name_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().to_string());
            res.anime.jname = document
                .select(jname_selector)
                .next()
                .and_then(|el| el.attr("data-jname"))
                .map(|s| s.trim().to_string());
            res.anime.description = document
                .select(description_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().to_string());
            res.anime.poster = document
                .select(poster_selector)
                .next()
                .and_then(|el| el.attr("src"))
                .map(|s| s.trim().to_string());
        }

        // seasons
        for el in document.select(seasons_selector) {
            res.anime.seasons.push(Season {
                id: el.attr("href").map(|s| s[1..].trim().to_string()),
                name: el.attr("title").map(|s| s.trim().to_string()),
                title: el
                    .select(season_title_selector)
                    .next()
                    .and_then(|el| el.text().next())
                    .map(|s| s.trim().to_string()),
                poster: el
                    .select(season_poster_selector)
                    .next()
                    .and_then(|el| el.attr("style"))
                    .and_then(|s| s.split(" ").last())
                    .and_then(|s| s.split("(").last())
                    .and_then(|s| s.split(")").next())
                    .map(|s| s.to_string()),
                is_current: el.value().classes().any(|class| class == "active"),
            });
        }

        // stats
        {
            res.anime.stats.rating = document
                .select(rating_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().to_string());
            res.anime.stats.quality = document
                .select(quality_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().to_string());
            res.anime.stats.episodes.sub = document
                .select(sub_episodes_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().parse::<u16>().ok())
                .flatten();
            res.anime.stats.episodes.dub = document
                .select(dub_episodes_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().parse::<u16>().ok())
                .flatten();

            let re = Regex::new(r"\s+").map_err(|_| {
                EnmaError::parsing_error(
                    PROVIDER_PARSER,
                    Some(String::from("error pasring regex")),
                    None,
                )
            })?;

            if let Some(el) = document.select(other_stats_selector).next() {
                let other_info = re
                    .replace_all(el.text().collect::<Vec<_>>().concat().trim(), " ")
                    .into_owned();
                let other_infos = other_info.split(" ").collect::<Vec<_>>();

                res.anime.stats.duration = other_infos.last().map(|s| s.to_string());
                res.anime.stats.anime_type = Some(other_infos[other_infos.len() - 2].to_string());
            }
        }

        // anime other info
        for el in document.select(other_info_selector) {
            let key = el
                .select(other_info_key_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.to_lowercase().replace(":", ""))
                .map(|s| {
                    if s.contains(" ") {
                        s.replace(" ", "")
                    } else {
                        s
                    }
                })
                .unwrap_or_default();

            let value = el
                .select(other_info_value_selector)
                .map(|el| el.text().collect::<Vec<_>>().concat().to_string())
                .collect::<Vec<_>>()
                .join(", ");

            match key.as_str() {
                "genres" | "producers" => {
                    res.anime.other_info.insert(
                        key,
                        OtherInfoValue::Multiple(
                            value
                                .split(", ")
                                .map(|s| s.to_string())
                                .collect::<Vec<String>>(),
                        ),
                    );
                }
                _ => {
                    res.anime
                        .other_info
                        .insert(key, OtherInfoValue::Single(value));
                }
            }
        }

        // promotional videos
        for el in document.select(promotional_videos_selector) {
            res.anime.promotional_videos.push(AnimePromotionalVideo {
                title: el.attr("data-title").map(|s| s.trim().to_string()),
                source: el.attr("data-src").map(|s| s.trim().to_string()),
                thumbnail: el
                    .select(promotional_video_thumbnail_selector)
                    .next()
                    .and_then(|el| el.attr("src"))
                    .map(|s| s.trim().to_string()),
            });
        }

        // character and voice actors
        for el in document.select(char_vc_actor_selector) {
            res.anime
                .characters_voice_actors
                .push(AnimeCharactersVoiceActors {
                    character: AnimeCharacter {
                        id: el
                            .select(char_id_selector)
                            .next()
                            .and_then(|el| el.attr("href"))
                            .and_then(|s| {
                                let vals = s.trim().split("/").collect::<Vec<_>>();
                                if vals.len() < 3 {
                                    return None;
                                }
                                return Some(vals[2].to_string());
                            }),
                        name: el
                            .select(char_name_selector)
                            .next()
                            .and_then(|el| el.text().next())
                            .map(|s| s.trim().to_string()),
                        cast: el
                            .select(char_cast_selector)
                            .next()
                            .and_then(|el| el.text().next())
                            .map(|s| s.trim().to_string()),
                        poster: el
                            .select(char_poster_selector)
                            .next()
                            .and_then(|el| el.attr("data-src"))
                            .map(|s| s.trim().to_string()),
                        ..Default::default()
                    },
                    voice_actor: AnimeCharacter {
                        id: el
                            .select(vc_actor_id_selector)
                            .next()
                            .and_then(|el| el.attr("href"))
                            .and_then(|s| {
                                let vals = s.trim().split("/").collect::<Vec<_>>();
                                if vals.len() < 3 {
                                    return None;
                                }
                                return Some(vals[2].to_string());
                            }),
                        name: el
                            .select(vc_actor_name_selector)
                            .next()
                            .and_then(|el| el.text().next())
                            .map(|s| s.trim().to_string()),
                        cast: el
                            .select(vc_actor_cast_selector)
                            .next()
                            .and_then(|el| el.text().next())
                            .map(|s| s.trim().to_string()),
                        poster: el
                            .select(vc_actor_poster_selector)
                            .next()
                            .and_then(|el| el.attr("data-src"))
                            .map(|s| s.trim().to_string()),
                    },
                });
        }

        // other content
        res.related_animes =
            HiAnimeUtils::extract_most_popular_anime(&document, related_anime_selector);
        res.most_popular_animes =
            HiAnimeUtils::extract_most_popular_anime(&document, most_popular_selector);
        res.recommended_animes =
            HiAnimeUtils::extract_animes(&document, recommended_anime_selector);

        let (mal_id, anilist_id) = HiAnimeUtils::get_mal_anilist_id(&document);
        res.anime.mal_id = mal_id;
        res.anime.anilist_id = anilist_id;

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::anime::hianime;
    use serde_json::to_string_pretty;

    // cargo test --lib -- anime::hianime::parsers::info::test --show-output
    #[tokio::test]
    async fn test_get_info() {
        let hianime = hianime::Scraper::new();

        let anime_id = "one-piece-100";

        match hianime.get_info(anime_id).await {
            // Ok(_) => (),
            Ok(data) => {
                println!("{}", to_string_pretty(&data).unwrap());

                assert_eq!(data.anime.id, Some(anime_id.to_string()));
                assert_eq!(data.anime.name, Some(String::from("One Piece")));
                assert_eq!(data.anime.mal_id, Some(21));

                assert_ne!(data.anime.other_info.len(), 0);
                assert_ne!(data.anime.characters_voice_actors.len(), 0);
                assert_ne!(data.recommended_animes.len(), 0);
                assert_ne!(data.related_animes.len(), 0);
                assert_ne!(data.most_popular_animes.len(), 0);
            }
            Err(e) => eprintln!("error {}", e),
        }
    }
}
