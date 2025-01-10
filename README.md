<img
  src="https://raw.githubusercontent.com/ghoshRitesh12/aniwatch-api/refs/heads/main/public/img/hianime_v2.png"
  alt="logo"
  width="175"
  align="left"
/>

# enma.rs

A crate serving anime and manga information 📦

> enma.rs is basically a rust port and a super set of the [aniwatch scraper package](https://github.com/ghoshRitesh12/aniwatch), which is internally used by the [aniwatch-api](https://github.com/ghoshRitesh12/aniwatch-api).
> <br/>

> [!IMPORTANT]
>
> 1. This package is just an unofficial package for the different providers of anime and manga and is in no other way officially related to the same.
> 2. The content that this package provides is not mine, nor is it hosted by me. These belong to their respective owners. This package just demonstrates how to build a package that scrapes websites and uses their content.

## Table of Contents

- [Quick Start](#quick-start)
  - [Installation](#installation)
  - [Example Usage](#example-usage)

## Quick start

### Installation

To use the `enma` crate, run the following command in your project directory:

```bash
cargo add enma
```

### Example usage

Example - getting information about an anime category by providing the name of the category and page number(optional); using anime category `most-favorite` with page number `2` as an example.

```rust
async fn get_data() {
    use enma::anime::hianime;
    let hianime = hianime::Scraper::new();

    let category = "most-favorite";
    let page_number = Some(2);

    match hianime.get_category_anime(category, page_number).await {
        Ok(data) => println!("{data:#?}"),
        Err(e) => eprintln!("error: {e}"),
    }
}
```
