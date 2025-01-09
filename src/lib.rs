//!  <img
//!    src="https://raw.githubusercontent.com/ghoshRitesh12/aniwatch-api/refs/heads/main/public/img/hianime_v2.png"
//!    alt="logo"
//!    width="150"
//!  />
//!
//!  # enma.rs
//!
//!  A crate serving anime and manga information 📦
//!
//!  > enma.rs is basically a rust port and a super set of the
//!  > [aniwatch scraper package](https://github.com/ghoshRitesh12/aniwatch),
//!  > which is internally used by the [aniwatch-api](https://github.com/ghoshRitesh12/aniwatch-api).
//!  <br/>
//!
//!  > [!IMPORTANT]
//!  >
//!  > 1. This package is just an unofficial package for the different providers of anime and manga and is in no other way officially related to the same.
//!  > 2. The content that this package provides is not mine, nor is it hosted by me. These belong to their respective owners. This package just demonstrates how to build a package that scrapes websites and uses their content.
//!
//!  ## Table of Contents
//!
//!  - [Quick Start](#quick-start)
//!    - [Installation](#installation)
//!    - [Example Usage](#example-usage)
//!      <!-- - [Documentation](#documentation) -->
//!        <!-- - [getHomePage](#gethomepage)
//!    - [getAZList](#getazlist)
//!    - [getQtipInfo](#getqtipinfo)
//!    - [getAnimeAboutInfo](#getanimeaboutinfo)
//!    - [getAnimeSearchResults](#getanimesearchresults)
//!    - [getAnimeSearchSuggestion](#getanimesearchsuggestion)
//!    - [getProducerAnimes](#getproduceranimes)
//!    - [getGenreAnime](#getgenreanime)
//!    - [getAnimeCategory](#getanimecategory)
//!    - [getEstimatedSchedule](#getestimatedschedule)
//!    - [getAnimeEpisodes](#getanimeepisodes)
//!    - [getEpisodeServers](#getepisodeservers)
//!    - [getAnimeEpisodeSources](#getanimeepisodesources) -->
//!      <!-- - [Development](#development) -->
//!      <!-- - [Thanks](#thanks) -->
//!      <!-- - [Support](#support) -->
//!      <!-- - [License](#license) -->
//!      <!-- - [Contributors](#contributors) -->
//!      <!-- - [Star History](#star-history) -->
//!  ## Quick start
//!
//!  ### Installation

//!  To use the `enma` crate, run the following command in your project directory:

//!  ```bash
//!  cargo add enma
//!  ```

//!  ### Example usage

//!  Example - getting information about an anime category by providing the name of the category and page number(optional); using anime category `most-favorite` with page number `2` as an example.

//!  ```rust
//!  use enma::anime::hianime;
//!
//!  async fn get_data() {
//!      let hianime = hianime::Scraper::new();
//!
//!      let category = "most-favorite";
//!      let page_number = Some(2);
//!
//!      match hianime.get_category_anime(category, page_number).await {
//!          Ok(data) => println!("{data:#?}"),
//!          Err(e) => eprintln!("error: {e}"),
//!      }
//!  }
//!  ```
//!

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
