use std::collections::{HashMap, HashSet};

mod utils;

mod parsers;
mod types;
use once_cell::sync::Lazy;
pub use parsers::*;

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
