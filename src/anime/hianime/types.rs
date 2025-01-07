#![allow(dead_code)]

use once_cell::sync::Lazy;
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Episodes {
    pub sub: Option<u16>,
    pub dub: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Top10AnimesWithPeriod {
    pub today: Vec<Top10Anime>,
    pub week: Vec<Top10Anime>,
    pub month: Vec<Top10Anime>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
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

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MostPopularAnime {
    pub id: Option<String>,
    pub name: Option<String>,
    pub poster: Option<String>,
    pub jname: Option<String>,
    #[serde(rename = "type")]
    pub anime_type: Option<String>,
    pub episodes: Episodes,
}

#[derive(Serialize, Deserialize, Debug, Default)]
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

#[derive(Serialize, Deserialize, Debug, Default)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Season {
    pub id: Option<String>,
    pub name: Option<String>,
    pub poster: Option<String>,
    pub title: Option<String>,
    pub is_current: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeGeneralInfo {
    pub id: Option<String>,
    pub name: Option<String>,
    pub poster: Option<String>,
    pub description: Option<String>,
    pub anilist_id: Option<u32>,
    pub mal_id: Option<u32>,

    pub stats: AnimeGeneralStats,
    pub promotional_videos: Vec<AnimePromotionalVideo>,
    pub characters_voice_actors: Vec<AnimeCharactersVoiceActors>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeGeneralStats {
    pub quality: Option<String>,
    pub duration: Option<String>,
    pub rating: Option<String>,
    #[serde(rename = "type")]
    pub anime_type: Option<String>,
    pub episodes: Episodes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimePromotionalVideo {
    pub title: Option<String>,
    pub source: Option<String>,
    pub thumbnail: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeCharactersVoiceActors {
    pub character: AnimeCharacter,
    pub voice_actor: AnimeCharacter,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeCharacter {
    pub id: Option<String>,
    pub name: Option<String>,
    pub poster: Option<String>,
    pub cast: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AnimeSearchSuggestion {
    pub id: Option<String>,
    pub name: Option<String>,
    pub poster: Option<String>,
    pub jname: Option<String>,
    pub other_info: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeEpisode {
    pub title: Option<String>,
    pub number: Option<i32>,
    pub episode_id: Option<String>,
    pub is_filler: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EpisodeInfo {
    pub server_name: Option<String>,
    pub server_id: Option<String>,
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

pub static ANIME_SERVERS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    return ["hd-1", "hd-2", "megacloud", "streamsb", "streamtape"]
        .into_iter()
        .collect();
});

#[derive(Serialize, Deserialize, Debug, Default)]
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

// #[derive(Debug, Serialize, Deserialize)]
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
