use crate::{
    anime::hianime::{
        parsers::types::ScrapedSchedule, types::ScheduledAnime, utils::HiAnimeUtils, Scraper,
    },
    error::EnmaResult,
    utils::{EnmaClient, EnmaUtils},
};
use chrono::{DateTime, Utc};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, REFERER};
use scraper::{Html, Selector};
use serde::Deserialize;

/// represents the raw data received from the source
#[derive(Deserialize, Debug)]
struct RawData {
    html: Option<String>,
}

impl Scraper {
    pub async fn get_schedule(&self, year: u16, month: u8, day: u8) -> EnmaResult<ScrapedSchedule> {
        const PROVIDER_PARSER: &'static str = "hianime:get_schedule";
        const NO_DATA_INDICATOR: &'static str = "No data to display";

        let mut res = ScrapedSchedule::default();
        let formatted_date = format!("{}-{:02}-{:02}", year, month, day);

        let url = format!(
            "{}?tzOffset=-330&date={}",
            HiAnimeUtils::ScheduleUrl.value(),
            formatted_date
        );
        let headers: HeaderMap = [
            (ACCEPT, HeaderValue::from_static("*/*")),
            (
                REFERER,
                HeaderValue::from_static(HiAnimeUtils::HomeUrl.value()),
            ),
            (
                HeaderName::from_static("x-requested-with"),
                HeaderValue::from_static(EnmaUtils::XRequestedWithHeader.value()),
            ),
        ]
        .into_iter()
        .collect();

        let data = self
            .client
            .get_json::<RawData>(url, Some(headers), PROVIDER_PARSER)
            .await?;

        let document = Html::parse_fragment(data.html.unwrap_or_default().as_str());
        let selector = &Selector::parse("li").unwrap();

        if let Some(data) = document
            .select(selector)
            .next()
            .and_then(|el| el.text().next())
            .map(|s| s.trim())
        {
            if data.contains(NO_DATA_INDICATOR) {
                return Ok(res);
            }
        }

        let id_selector = &Selector::parse("li a.tsl-link").unwrap();
        let name_selector = &Selector::parse("li a .film-name.dynamic-name").unwrap();
        let jname_selector = name_selector;
        let episode_no_selector = &Selector::parse("li a .fd-play button.btn-play").unwrap();
        let time_selector = &Selector::parse("li a.tsl-link .time").unwrap();

        for el in document.select(selector) {
            let id = el
                .select(id_selector)
                .next()
                .and_then(|el| el.attr("href"))
                .map(|href| href.trim()[1..].to_string());

            let name = el
                .select(name_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().to_string());

            let jname = el
                .select(jname_selector)
                .next()
                .and_then(|el| el.attr("data-jname"))
                .map(|s| s.trim().to_string());

            let episode_number = document
                .select(episode_no_selector)
                .next()
                .and_then(|el| el.text().next())
                .and_then(|s| s.trim().split(" ").last())
                .and_then(|s| s.trim().parse::<u16>().ok());

            let airing_time = el
                .select(time_selector)
                .next()
                .and_then(|el| el.text().next())
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
            let datetime_str = format!("{formatted_date}T{airing_time}:00+00:00");

            let airing_timestamp = DateTime::parse_from_rfc3339(&datetime_str)
                .unwrap_or_default()
                .with_timezone(&Utc)
                .timestamp_millis();

            let seconds_until_airing =
                ((airing_timestamp - Utc::now().timestamp_millis()) / 1000) as i32;

            res.animes.push(ScheduledAnime {
                id,
                name,
                jname,
                episode_number,
                airing_timestamp,
                seconds_until_airing,
                time: Some(airing_time),
                already_aired: seconds_until_airing <= 0,
            });
        }

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::anime::hianime;
    use chrono::{Datelike, Utc};
    use serde_json::to_string_pretty;

    // cargo test --lib -- anime::hianime::parsers::schedule::test --show-output
    #[tokio::test]
    async fn test_get_schedule() {
        let hianime = hianime::Scraper::new();

        let now = Utc::now();
        let (year, month, day) = (now.year() as u16, now.month() as u8, now.day() as u8);

        match hianime.get_schedule(year, month, day).await {
            // Ok(_) => (),
            Ok(data) => {
                println!("{}", to_string_pretty(&data).unwrap());

                assert_ne!(data.animes.len(), 0);
            }
            Err(e) => eprintln!("error {}", e),
        }
    }
}
