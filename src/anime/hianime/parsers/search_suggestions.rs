use super::{HiAnimeUtils, Scraper};

impl Scraper {
    pub async fn get_search_suggestions(&self, q: &str) {
        // self.client.get(SRC_SEARCH_URL);
        println!(
            "{} {} {}",
            HiAnimeUtils::AjaxUrl.value(),
            HiAnimeUtils::SearchUrl.value(),
            q
        );
        println!("search_suggestions")
    }
}
