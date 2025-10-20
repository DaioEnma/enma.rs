use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Default)]
pub struct ExtractedData {
    pub subtitles: Vec<Subtitle>,
    pub intro: Option<Intro>,
    pub outro: Option<Intro>,
    pub sources: Vec<Source>,
}

#[derive(Serialize, Debug, Default)]
pub struct Subtitle {
    pub url: String,
    pub lang: String,
    pub default: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Intro {
    pub start: f64,
    pub end: f64,
}

#[derive(Serialize, Debug, Default)]
pub struct Source {
    pub url: String,
    pub isM3U8: bool,
    pub quality: Option<String>,
}

#[derive(Deserialize)]
pub struct MegacloudKeysRepoResponse {
    pub mega: String,
}

#[derive(Deserialize, Debug)]
pub struct DecryptedSourceData {
    pub sources: DecryptedSourceDataSourceType,
    pub tracks: Vec<DecryptedSourceDataTrack>,
    pub encrypted: bool,
    pub intro: Option<Intro>,
    pub outro: Option<Intro>,
    pub server: u8,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum DecryptedSourceDataSourceType {
    Sources(Vec<DecryptedSourceDataSource>),
    Encrypted(String),
}

#[derive(Deserialize, Debug)]
pub struct DecryptedSourceDataSource {
    pub file: String,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct DecryptedSourceDataTrack {
    pub file: String,
    pub label: Option<String>,
    pub kind: String,
    pub default: Option<bool>,
}