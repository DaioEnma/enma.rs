use super::{Scraper, SRC_HOME_URL};
use crate::error::{EnmaError, EnmaResult};

impl Scraper {
    pub async fn get_home_page(&self) -> EnmaResult<String> {
        let provider_parser: &'static str = "HiAnime:get_home_page";

        let resp = self
            .client
            .get(SRC_HOME_URL)
            .send()
            .await
            .map_err(|_| EnmaError::src_fetch_error(provider_parser, None, None));

        let home_page_html = resp?
            .text()
            .await
            .map_err(|_| EnmaError::src_parse_error(provider_parser, None, None))?;

        return Ok(home_page_html);
    }
}

#[cfg(test)]
mod test {
    use crate::anime::hianime;

    // cargo test --lib -- anime::hianime::parsers::home_page::test --show-output
    #[tokio::test]
    async fn test_get_home_page() {
        let hianime = hianime::Scraper::new();
        match hianime.get_home_page().await {
            Ok(data) => println!("{data}"),
            Err(e) => eprintln!("error {}", e),
        }
    }
}
