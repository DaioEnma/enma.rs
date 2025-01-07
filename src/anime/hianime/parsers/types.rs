use crate::anime::hianime::types::{
    Anime, AnimeSearchSuggestion, LatestCompletedAnime, LatestEpisodeAnime, MostFavoriteAnime,
    MostPopularAnime, QtipAnime, SpotlightAnime, Top10AnimesWithPeriod, TopAiringAnime,
    TopUpcomingAnime, TrendingAnime,
};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScrapedSearchSuggestion {
    pub suggestions: Vec<AnimeSearchSuggestion>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScrapedAZList {
    pub animes: Vec<Anime>,
    pub sort_option: &'static str,
    pub total_pages: u16,
    pub current_page: u16,
    pub has_next_page: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScrapedQtipInfo {
    pub anime: QtipAnime,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScrapedCategoryAnime {
    pub category: String,
    pub animes: Vec<Anime>,
    pub total_pages: u16,
    pub current_page: u16,
    pub has_next_page: bool,
    pub genres: Vec<String>,
    pub top10_animes: Top10AnimesWithPeriod,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScrapedProducerAnime {
    pub producer: String,
    pub animes: Vec<Anime>,
    pub total_pages: u16,
    pub current_page: u16,
    pub has_next_page: bool,
    pub top_airing_animes: Vec<TopAiringAnime>,
    pub top10_animes: Top10AnimesWithPeriod,
}
