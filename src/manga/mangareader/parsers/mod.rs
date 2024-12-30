#[derive(Debug)]
pub struct Scraper {}

const SRC_BASE_URL: &str = "https://mangareader.to";
const SRC_HOME_URL: &str = "https://mangareader.to/home";

impl Scraper {
    pub fn new() -> Scraper {
        println!("{}{}", SRC_BASE_URL, SRC_HOME_URL);
        return Scraper {};
    }
}
