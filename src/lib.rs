#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/ghoshRitesh12/aniwatch-api/refs/heads/main/public/img/hianime_v2.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ghoshRitesh12/aniwatch-api/refs/heads/main/public/img/hianime_v2.png"
)]
#![doc(issue_tracker_base_url = "https://github.com/DaioEnma/enma.rs/issues")]
#![doc = include_str!("../README.md")]

mod utils;

/// namespace for anime scraper providers
///
/// example import: `use enma::anime;`
pub mod anime;

/// namespace for manga scraper providers
///
/// example import: `use enma::manga;`
pub mod manga;

mod error;
pub use error::{EnmaError, EnmaResult};
