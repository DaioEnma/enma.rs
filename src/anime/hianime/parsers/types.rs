use serde::{Deserialize, Serialize};

use crate::anime::hianime::types::{
    LatestCompletedAnime, LatestEpisodeAnime, MostFavoriteAnime, MostPopularAnime, SpotlightAnime,
    Top10AnimesWithPeriod, TopAiringAnime, TopUpcomingAnime, TrendingAnime,
};

// pub trait Jsonify {
//     fn to_json(&self, provider_parser: &'static str) -> EnmaResult<String>;
// }

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScrapedHomePage {
    pub genres: Vec<String>,
    pub top10_animes: Top10AnimesWithPeriod,
    pub spotlight_animes: Vec<SpotlightAnime>,
    pub trending_animes: Vec<TrendingAnime>,
    pub latest_episode_animes: Vec<LatestEpisodeAnime>,
    pub top_upcoming_animes: Vec<TopUpcomingAnime>,
    pub top_airing_animes: Vec<TopAiringAnime>,
    pub most_popular_animes: Vec<MostPopularAnime>,
    pub most_favorite_animes: Vec<MostFavoriteAnime>,
    pub latest_completed_animes: Vec<LatestCompletedAnime>,
}

// impl Jsonify for ScrapedHomePage {
//     fn to_json(&self, provider_parser: &'static str) -> EnmaResult<String> {
//         match to_string(self) {
//             Ok(stringified_json) => Ok(stringified_json),
//             Err(_) => Err(EnmaError::json_serde_error(provider_parser, None, None)),
//         }
//     }
// }
