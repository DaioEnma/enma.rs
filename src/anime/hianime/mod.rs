use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    fmt,
    hash::Hash,
};

mod parsers;
mod types;
mod utils;

pub use parsers::*;

use crate::EnmaError;

#[derive(Debug)]
pub struct SearchPageFilters {
    pub genres_id_map: HashMap<&'static str, u8>,
    pub type_id_map: HashMap<&'static str, u8>,
    pub status_id_map: HashMap<&'static str, u8>,
    pub rated_id_map: HashMap<&'static str, u8>,
    pub score_id_map: HashMap<&'static str, u8>,
    pub season_id_map: HashMap<&'static str, u8>,
    pub language_id_map: HashMap<&'static str, u8>,
    pub sort_id_map: HashMap<&'static str, &'static str>,
}

pub static SEARCH_PAGE_FILTERS: Lazy<SearchPageFilters> = Lazy::new(|| SearchPageFilters {
    genres_id_map: HashMap::from([
        ("action", 1),
        ("adventure", 2),
        ("cars", 3),
        ("comedy", 4),
        ("dementia", 5),
        ("demons", 6),
        ("drama", 8),
        ("ecchi", 9),
        ("fantasy", 10),
        ("game", 11),
        ("harem", 35),
        ("historical", 13),
        ("horror", 14),
        ("isekai", 44),
        ("josei", 43),
        ("kids", 15),
        ("magic", 16),
        ("martial-arts", 17),
        ("mecha", 18),
        ("military", 38),
        ("music", 19),
        ("mystery", 7),
        ("parody", 20),
        ("police", 39),
        ("psychological", 40),
        ("romance", 22),
        ("samurai", 21),
        ("school", 23),
        ("sci-fi", 24),
        ("seinen", 42),
        ("shoujo", 25),
        ("shoujo-ai", 26),
        ("shounen", 27),
        ("shounen-ai", 28),
        ("slice-of-life", 36),
        ("space", 29),
        ("sports", 30),
        ("super-power", 31),
        ("supernatural", 37),
        ("thriller", 41),
        ("vampire", 32),
    ]),
    type_id_map: HashMap::from([
        ("all", 0),
        ("movie", 1),
        ("tv", 2),
        ("ova", 3),
        ("ona", 4),
        ("special", 5),
        ("music", 6),
    ]),
    status_id_map: HashMap::from([
        ("all", 0),
        ("finished-airing", 1),
        ("currently-airing", 2),
        ("not-yet-aired", 3),
    ]),
    rated_id_map: HashMap::from([
        ("all", 0),
        ("g", 1),
        ("pg", 2),
        ("pg-13", 3),
        ("r", 4),
        ("r+", 5),
        ("rx", 6),
    ]),
    score_id_map: HashMap::from([
        ("all", 0),
        ("appalling", 1),
        ("horrible", 2),
        ("very-bad", 3),
        ("bad", 4),
        ("average", 5),
        ("fine", 6),
        ("good", 7),
        ("very-good", 8),
        ("great", 9),
        ("masterpiece", 10),
    ]),
    season_id_map: HashMap::from([
        ("all", 0),
        ("spring", 1),
        ("summer", 2),
        ("fall", 3),
        ("winter", 4),
    ]),
    language_id_map: HashMap::from([("all", 0), ("sub", 1), ("dub", 2), ("sub-&-dub", 3)]),
    sort_id_map: HashMap::from([
        ("default", "default"),
        ("recently-added", "recently_added"),
        ("recently-updated", "recently_updated"),
        ("score", "score"),
        ("name-a-z", "name_az"),
        ("released-date", "released_date"),
        ("most-watched", "most_watched"),
    ]),
});

pub static AZ_LIST_SORT_OPTIONS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    return [
        "all", "other", "0-9", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
        "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
    ]
    .into_iter()
    .collect();
});

/// enables search filters creation
#[derive(Debug, Default, Clone)]
pub struct SearchFilters {
    filters: HashSet<SearchFilter>,
}

impl SearchFilters {
    pub fn new(filters: Vec<SearchFilter>) -> SearchFilters {
        return SearchFilters {
            filters: filters
                .into_iter()
                // .map(|s| (s.clone(), s.value()))
                .collect(),
        };
    }

    fn to_query_params(&self) -> String {
        // self.filters.remove(&SearchFilter::Invalid.to_string());

        self.filters
            .iter()
            .filter_map(|s| {
                let value = s.mapped_value();
                if value.is_empty() {
                    return None;
                }

                if s.to_string().contains("_date") {
                    return Some(value);
                }
                return Some(format!("{}={}", s.to_string(), value));
            })
            .collect::<Vec<String>>()
            .join("&")
    }

    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }
}

impl Serialize for SearchFilters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let map: HashMap<String, String> = self
            .filters
            .iter()
            .map(|s| (s.to_string(), s.value()))
            .collect();

        map.serialize(serializer)
    }
}

/// different types of seach filter
#[derive(Serialize, Debug, Eq, Clone)]
#[serde(untagged)]
pub enum SearchFilter {
    Type(String),
    Status(String),
    Rated(String),
    Score(String),
    Season(String),
    Language(String),
    StartDate(String),
    EndDate(String),
    Sort(String),
    Genres(String),
    /// allows for invalid filter value, this doesn't affect
    /// the intended behavior as much, as it'd be typically
    /// ignored by the source
    Invalid,
}

impl SearchFilter {
    /// checks whether given filter is valid
    pub fn is_valid(filter_key: &'static str) -> bool {
        match filter_key {
            "type" => true,
            "status" => true,
            "rated" => true,
            "score" => true,
            "season" => true,
            "language" => true,
            "start_date" => true,
            "end_date" => true,
            "sort" => true,
            "genres" => true,
            _ => false,
        }
    }

    /// converts raw query string and its value to [SearchFilter] enum
    pub fn from_raw(filter_key: &'static str, filter_value: String) -> Self {
        match filter_key {
            "type" => Self::Type(filter_value),
            "status" => Self::Status(filter_value),
            "rated" => Self::Rated(filter_value),
            "score" => Self::Score(filter_value),
            "season" => Self::Season(filter_value),
            "language" => Self::Language(filter_value),
            "start_date" => Self::StartDate(filter_value),
            "end_date" => Self::EndDate(filter_value),
            "sort" => Self::Sort(filter_value),
            "genres" => Self::Genres(filter_value),
            _ => Self::Invalid,
        }
    }

    /// converts variant to its representable query params key
    fn to_string(&self) -> String {
        format!("{self}")
    }

    /// gets the mapped value of the variant
    fn mapped_value(&self) -> String {
        match self {
            SearchFilter::Type(value) => SEARCH_PAGE_FILTERS
                .type_id_map
                .get(value.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default(),
            SearchFilter::Status(value) => SEARCH_PAGE_FILTERS
                .status_id_map
                .get(value.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default(),
            SearchFilter::Rated(value) => SEARCH_PAGE_FILTERS
                .rated_id_map
                .get(value.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default(),
            SearchFilter::Score(value) => SEARCH_PAGE_FILTERS
                .score_id_map
                .get(value.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default(),
            SearchFilter::Season(value) => SEARCH_PAGE_FILTERS
                .season_id_map
                .get(value.as_str())
                .map(|s| s.to_string())
                .unwrap_or(String::from("default")),
            SearchFilter::Language(value) => SEARCH_PAGE_FILTERS
                .language_id_map
                .get(value.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default(),
            SearchFilter::StartDate(value) | SearchFilter::EndDate(value) => {
                return SearchFilter::get_date_filter_value(value, &self.to_string());
            }
            SearchFilter::Sort(value) => SEARCH_PAGE_FILTERS
                .sort_id_map
                .get(value.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default(),
            SearchFilter::Genres(value) => SearchFilter::get_genres_filter_value(
                value.split(",").map(|s| s.to_string()).collect::<Vec<_>>(),
            )
            .unwrap_or_default(),
            SearchFilter::Invalid => String::from(""),
        }
    }

    // gets the value of the variant
    fn value(&self) -> String {
        let invalid_value = String::from("_invalid_value_");

        match self {
            SearchFilter::Type(value) => value.into(),
            SearchFilter::Status(value) => value.into(),
            SearchFilter::Rated(value) => value.into(),
            SearchFilter::Score(value) => value.into(),
            SearchFilter::Season(value) => value.into(),
            SearchFilter::Language(value) => value.into(),
            SearchFilter::StartDate(value) | SearchFilter::EndDate(value) => value.into(),
            SearchFilter::Sort(value) => value.into(),
            SearchFilter::Genres(value) => value.into(),
            SearchFilter::Invalid => invalid_value,
        }
    }

    fn get_date_filter_value(raw_value: &String, category: &String) -> String {
        let date_regex = Regex::new(r"^\d{4}-([0-9]|1[0-2])-([0-9]|[12][0-9]|3[01])$")
            .map_err(|_| {
                EnmaError::parsing_error(
                    "hianime:get_search_results",
                    Some(String::from("couldn't parse regex")),
                    None,
                )
            })
            .unwrap();
        if !date_regex.is_match(&raw_value) {
            return String::from("");
        }

        let date_category = if category.starts_with("start") {
            "s"
        } else {
            "e"
        };

        let period = raw_value.split('-').collect::<Vec<_>>();
        let (year, month, day) = (
            period[0].parse::<u16>().unwrap_or_default(),
            period[1].parse::<u8>().unwrap_or_default(),
            period[2].parse::<u8>().unwrap_or_default(),
        );

        format!(
            "{date_category}y={}&{date_category}m={}&{date_category}d={}",
            year, month, day
        )
    }

    fn get_genres_filter_value(genre_names: Vec<String>) -> Option<String> {
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

impl fmt::Display for SearchFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchFilter::Type(_) => write!(f, "type"),
            SearchFilter::Status(_) => write!(f, "status"),
            SearchFilter::Rated(_) => write!(f, "rated"),
            SearchFilter::Score(_) => write!(f, "score"),
            SearchFilter::Season(_) => write!(f, "season"),
            SearchFilter::Language(_) => write!(f, "language"),
            SearchFilter::StartDate(_) => write!(f, "start_date"),
            SearchFilter::EndDate(_) => write!(f, "end_date"),
            SearchFilter::Sort(_) => write!(f, "sort"),
            SearchFilter::Genres(_) => write!(f, "genres"),
            SearchFilter::Invalid => write!(f, "_invalid_"),
        }
    }
}

impl Hash for SearchFilter {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl PartialEq for SearchFilter {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
