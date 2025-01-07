use crate::{
    anime::hianime::{utils::HiAnimeUtils, ScrapedAZList, Scraper, AZ_LIST_SORT_OPTIONS},
    utils::EnmaClient,
    EnmaError, EnmaResult,
};
use reqwest::StatusCode;
use scraper::{Html, Selector};

impl Scraper {
    pub async fn get_az_list(
        &self,
        sort_option: &'static str,
        page_number: Option<u16>,
    ) -> EnmaResult<ScrapedAZList> {
        const PROVIDER_PARSER: &'static str = "hianime:get_az_list";

        let sort_option = sort_option.trim();
        if !AZ_LIST_SORT_OPTIONS.contains(sort_option) {
            return Err(EnmaError::misc_error(
                PROVIDER_PARSER,
                Some(String::from("invalid az list option")),
                Some(StatusCode::BAD_REQUEST),
            ));
        }

        let mut res = ScrapedAZList {
            sort_option,
            current_page: page_number.unwrap_or(1).max(1),
            ..Default::default()
        };

        let sort_option = match sort_option {
            "all" => "".to_string(),
            "other" => "other".to_string(),
            _ => sort_option.to_uppercase(),
        };
        let url = format!(
            "{}/{sort_option}?page={}",
            HiAnimeUtils::AZListUrl.value(),
            &res.current_page
        );

        let selector =
            &Selector::parse("#main-wrapper .tab-content .film_list-wrap .flw-item").unwrap();

        let page = self.client.get_html(url, None, PROVIDER_PARSER).await?;
        let document = Html::parse_document(&page);

        res.has_next_page = HiAnimeUtils::has_next_page(&document);
        res.total_pages = HiAnimeUtils::get_total_pages(&document);
        res.animes = HiAnimeUtils::extract_animes(&document, selector);

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::anime::hianime;
    use serde_json::to_string_pretty;

    // cargo test --lib -- anime::hianime::parsers::az_list::test --show-output
    #[tokio::test]
    async fn test_get_az_list() {
        let hianime = hianime::Scraper::new();
        let sort_option = "e";
        let page_number = Some(2);

        match hianime.get_az_list(sort_option, page_number).await {
            // Ok(_) => (),
            Ok(data) => println!("{}", to_string_pretty(&data).unwrap()),
            Err(e) => eprintln!("{}", e),
        }
    }
}
