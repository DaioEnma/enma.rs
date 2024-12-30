use super::{Scraper, SRC_AJAX_URL, SRC_BASE_URL, SRC_HOME_URL, SRC_SEARCH_URL};

impl Scraper {
    pub fn get_search_suggestions(&self, q: &str) {
        // self.client.get(SRC_SEARCH_URL);
        println!(
            "{}{}{}{} {}",
            SRC_SEARCH_URL, SRC_AJAX_URL, SRC_BASE_URL, SRC_HOME_URL, q
        );
        println!("search_suggestions")
    }
}
