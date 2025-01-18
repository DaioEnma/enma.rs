use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

use serde::Serialize;

#[derive(Serialize, Debug, Default)]
pub struct Anime {
    pub id: Option<String>,
    pub name: Option<String>,
    pub poster: Option<String>,
    /// japanese name
    pub jname: Option<String>,
    pub duration: Option<String>,
    pub rating: Option<String>,
    #[serde(rename = "type")]
    pub anime_type: Option<String>,
    pub episodes: Episodes,
}

#[derive(Serialize, Debug, Default)]
pub struct Episodes {
    pub sub: Option<u16>,
    pub dub: Option<u16>,
}

#[derive(Serialize, Debug, Default)]
pub struct Top10AnimesWithPeriod {
    pub today: Vec<Top10Anime>,
    pub week: Vec<Top10Anime>,
    pub month: Vec<Top10Anime>,
}

#[derive(Serialize, Debug, Default)]
pub struct Top10Anime {
    pub id: Option<String>,
    pub name: Option<String>,
    pub poster: Option<String>,
    pub jname: Option<String>,
    pub rank: Option<u32>,
    pub episodes: Episodes,
}

pub enum Top10AnimePeriod {
    Day,
    Week,
    Month,
}

impl Top10AnimePeriod {
    pub fn from(period: &String) -> Self {
        match period.as_str() {
            "day" => Self::Day,
            "week" => Self::Week,
            "month" => Self::Month,
            _ => Self::Day,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct MostPopularAnime {
    pub id: Option<String>,
    pub name: Option<String>,
    pub poster: Option<String>,
    pub jname: Option<String>,
    #[serde(rename = "type")]
    pub anime_type: Option<String>,
    pub episodes: Episodes,
}

#[derive(Serialize, Debug, Default)]
pub struct SpotlightAnime {
    pub id: Option<String>,
    pub name: Option<String>,
    pub poster: Option<String>,
    pub jname: Option<String>,
    #[serde(rename = "type")]
    pub anime_type: Option<String>,
    pub rank: Option<u32>,
    pub description: Option<String>,
    pub other_info: Vec<String>,
    pub episodes: Episodes,
}

#[derive(Serialize, Debug, Default)]
pub struct TrendingAnime {
    pub id: Option<String>,
    pub name: Option<String>,
    pub poster: Option<String>,
    pub jname: Option<String>,
    pub rank: Option<u32>,
}

pub type RecommendedAnime = Anime;
pub type TopUpcomingAnime = Anime;
pub type LatestEpisodeAnime = Anime;

pub type RelatedAnime = MostPopularAnime;
pub type TopAiringAnime = MostPopularAnime;
pub type MostFavoriteAnime = MostPopularAnime;
pub type LatestCompletedAnime = MostPopularAnime;

#[derive(Serialize, Debug, Default)]
pub struct Season {
    pub id: Option<String>,
    pub name: Option<String>,
    pub poster: Option<String>,
    pub title: Option<String>,
    pub is_current: bool,
}

#[derive(Debug, Serialize)]
#[serde(untagged)] // Serialize without including a tag in JSON
pub enum OtherInfoValue {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Serialize, Debug, Default)]
pub struct AnimeDetailedInfo {
    pub id: Option<String>,
    pub name: Option<String>,
    pub jname: Option<String>,
    pub poster: Option<String>,
    pub description: Option<String>,
    pub anilist_id: Option<u32>, // skeptical about it being u64
    pub mal_id: Option<u32>,     // skeptical about it being u64

    pub seasons: Vec<Season>,
    pub other_info: HashMap<String, OtherInfoValue>,
    pub stats: AnimeDetailedStats,
    pub promotional_videos: Vec<AnimePromotionalVideo>,
    pub characters_voice_actors: Vec<AnimeCharactersVoiceActors>,
}

#[derive(Serialize, Debug, Default)]
pub struct AnimeDetailedStats {
    pub quality: Option<String>,
    pub duration: Option<String>,
    pub rating: Option<String>,
    #[serde(rename = "type")]
    pub anime_type: Option<String>,
    pub episodes: Episodes,
}

#[derive(Serialize, Debug)]
pub struct AnimePromotionalVideo {
    pub title: Option<String>,
    pub source: Option<String>,
    pub thumbnail: Option<String>,
}

#[derive(Serialize, Debug, Default)]
pub struct AnimeCharactersVoiceActors {
    pub character: AnimeCharacter,
    pub voice_actor: AnimeCharacter,
}

#[derive(Serialize, Debug, Default)]
pub struct AnimeCharacter {
    pub id: Option<String>,
    pub name: Option<String>,
    pub poster: Option<String>,
    pub cast: Option<String>,
}

#[derive(Serialize, Debug, Default)]
pub struct AnimeSearchSuggestion {
    pub id: Option<String>,
    pub name: Option<String>,
    pub poster: Option<String>,
    pub jname: Option<String>,
    pub other_info: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct AnimeEpisode {
    pub title: Option<String>,
    pub number: Option<u16>,
    pub episode_id: Option<String>,
    pub is_filler: bool,
}

#[derive(Serialize, Debug)]
pub struct EpisodeInfo {
    pub server_name: Option<String>,
    pub server_id: Option<u16>,
}

pub type SubEpisode = EpisodeInfo;
pub type DubEpisode = EpisodeInfo;
pub type RawEpisode = EpisodeInfo;

pub static ANIME_CATEGORIES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    return [
        "most-favorite",
        "most-popular",
        "subbed-anime",
        "dubbed-anime",
        "recently-updated",
        "recently-added",
        "top-upcoming",
        "top-airing",
        "movie",
        "special",
        "ova",
        "ona",
        "tv",
        "completed",
    ]
    .into_iter()
    .collect();
});

#[allow(dead_code)] // TODO -> remove this
pub static ANIME_SERVERS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    return ["hd-1", "hd-2", "megacloud", "streamsb", "streamtape"]
        .into_iter()
        .collect();
});

#[derive(Serialize, Debug, Default)]
pub struct QtipAnime {
    pub id: Option<String>,
    pub name: Option<String>,
    pub mal_score: Option<f64>,
    pub quality: Option<String>,
    pub episodes: Episodes,
    #[serde(rename = "type")]
    pub anime_type: Option<String>,
    pub description: Option<String>,

    pub jname: Option<String>,
    pub synonyms: Option<String>,
    pub aired: Option<String>,
    pub status: Option<String>,
    pub genres: Vec<String>,
}

#[derive(Serialize, Debug, Default)]
pub struct ScheduledAnime {
    pub id: Option<String>,
    pub name: Option<String>,
    pub jname: Option<String>,
    pub time: Option<String>,
    pub episode_number: Option<u16>,
    pub airing_timestamp: i64,
    pub already_aired: bool,
    pub seconds_until_airing: i32,
}

// #[derive(Debug, Serialize, )]
#[allow(dead_code)] // TODO -> remove this
pub enum Server {
    VidStreaming,
    MegaCloud,
    StreamSB,
    StreamTape,
    VidCloud,
    AsianLoad,
    GogoCDN,
    MixDrop,
    UpCloud,
    VizCloud,
    MyCloud,
    FileMoon,
}

#[allow(dead_code)] // TODO -> remove this
impl Server {
    pub fn value(&self) -> &'static str {
        match self {
            Server::VidStreaming => "hd-1",
            Server::MegaCloud => "megacloud",
            Server::StreamSB => "streamsb",
            Server::StreamTape => "streamtape",
            Server::VidCloud => "hd-2",
            Server::AsianLoad => "asianload",
            Server::GogoCDN => "gogocdn",
            Server::MixDrop => "mixdrop",
            Server::UpCloud => "upcloud",
            Server::VizCloud => "vizcloud",
            Server::MyCloud => "mycloud",
            Server::FileMoon => "filemoon",
        }
    }
}
