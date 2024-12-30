use super::SRC_HOME_URL;

use super::Scraper;

impl Scraper {
    pub async fn get_home_page(&self) {
        self.client.get(SRC_HOME_URL).send().await.unwrap();
        println!("home_page")
    }
}
