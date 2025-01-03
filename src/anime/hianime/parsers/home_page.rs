use super::{Scraper, SRC_HOME_URL};
use crate::error::{EnmaError, EnmaResult};

impl Scraper {
    pub async fn get_home_page(&self) -> EnmaResult<String> {
        let provider_parser: &'static str = "HiAnime:get_home_page";

        let home_page_html = self.client.get(SRC_HOME_URL).send().await.map_err(|err| {
            return EnmaError::src_fetch_error(err, provider_parser, None);
        });

        let data = home_page_html?.text().await.map_err(|_| {
            return EnmaError::src_parse_error(provider_parser, None, None);
        })?;

        return Ok(data);
    }
}

#[cfg(test)]
mod test {
    use crate::anime::hianime;

    // cargo test  --lib -- anime::hianime::parsers::home_page::test --show-output
    #[tokio::test]
    async fn test_get_home_page() {
        let hianime = hianime::Scraper::new();
        match hianime.get_home_page().await {
            Ok(data) => println!("{data}"),
            Err(e) => eprintln!("error {}", e),
        }
    }
}
