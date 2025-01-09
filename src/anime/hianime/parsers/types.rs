use crate::anime::hianime::types::{
    Anime, AnimeDetailedInfo, AnimeEpisode, AnimeSearchSuggestion, DubEpisode,
    LatestCompletedAnime, LatestEpisodeAnime, MostFavoriteAnime, MostPopularAnime, QtipAnime,
    RawEpisode, RecommendedAnime, RelatedAnime, ScheduledAnime, SpotlightAnime, SubEpisode,
    Top10AnimesWithPeriod, TopAiringAnime, TopUpcomingAnime, TrendingAnime,
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
    pub category_name: String,
    pub animes: Vec<Anime>,
    pub total_pages: u16,
    pub current_page: u16,
    pub has_next_page: bool,
    pub genres: Vec<String>,
    pub top10_animes: Top10AnimesWithPeriod,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScrapedProducerAnime {
    pub producer_name: String,
    pub animes: Vec<Anime>,
    pub total_pages: u16,
    pub current_page: u16,
    pub has_next_page: bool,
    pub top_airing_animes: Vec<TopAiringAnime>,
    pub top10_animes: Top10AnimesWithPeriod,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScrapedGenreAnime {
    pub genre_name: String,
    pub animes: Vec<Anime>,
    pub genres: Vec<String>,
    pub total_pages: u16,
    pub current_page: u16,
    pub has_next_page: bool,
    pub top_airing_animes: Vec<TopAiringAnime>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScrapedAnimeEpisodes {
    pub total_episodes: Option<u16>,
    pub episodes: Vec<AnimeEpisode>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScrapedEpisodeServers {
    pub anime_episode_id: &'static str,
    pub episode_number: Option<u16>,
    pub sub: Vec<SubEpisode>,
    pub dub: Vec<DubEpisode>,
    pub raw: Vec<RawEpisode>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScrapedSchedule {
    pub animes: Vec<ScheduledAnime>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScrapedAnimeInfo {
    pub anime: AnimeDetailedInfo,
    pub related_animes: Vec<RelatedAnime>,
    pub recommended_animes: Vec<RecommendedAnime>,
    pub most_popular_animes: Vec<MostPopularAnime>,
}
