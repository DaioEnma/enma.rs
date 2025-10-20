use regex::Regex;
use reqwest::header::{HeaderValue, REFERER};

use crate::{anime::extractors::{megacloud_utils::decrypt_src, types::{DecryptedSourceData, DecryptedSourceDataSource, DecryptedSourceDataSourceType, ExtractedData, MegacloudKeysRepoResponse, Source, Subtitle}}, EnmaError, EnmaResult};

pub struct Megacloud {
    client: reqwest::Client,
}

impl Megacloud {
    pub fn new(client: reqwest::Client) -> Self {
        Self {
            client,
        }
    }
    
    pub async fn extract(&self, url: &str) -> EnmaResult<ExtractedData> {
        let response = self.client
            .get("https://raw.githubusercontent.com/yogesh-hacker/MegacloudKeys/refs/heads/main/keys.json")
            .send().await.expect("Failed to get megacloud key from repository");
        let megacloud_key = response.json::<MegacloudKeysRepoResponse>().await
            .expect("Failed to parse megacloud key json").mega;

        let re = Regex::new(r"/([^/?]+)\?").expect("Failed to create regex");
        let source_id = re.captures(url)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
            .expect("Failed to extract source ID from URL");

        let client_key = self.get_client_key(&source_id).await?;

        let response = self.client
            .get(format!("https://megacloud.blog/embed-2/v3/e-1/getSources?id={}&_k={}", source_id, client_key))
            .send().await.expect("Failed to get megacloud key from repository");
        let mut raw_sources = response.json::<DecryptedSourceData>().await.unwrap();
        
        if let DecryptedSourceDataSourceType::Encrypted(encrypted) = raw_sources.sources {
            let decrypted_payload = decrypt_src(&encrypted, &client_key, &megacloud_key).unwrap();
            let decrypted_data: Vec<DecryptedSourceDataSource> = serde_json::from_str(&decrypted_payload).unwrap();
            raw_sources.sources = DecryptedSourceDataSourceType::Sources(decrypted_data);
        }

        Ok(ExtractedData {
            subtitles: raw_sources.tracks.iter()
            .filter(|track| track.kind == "captions")
            .map(|track| {
                Subtitle {
                    url: track.file.clone(),
                    lang: track.label.clone().unwrap_or_default(),
                    default: track.default,
                }
            }).collect(),
            intro: raw_sources.intro,
            outro: raw_sources.outro,
            sources: match raw_sources.sources {
                DecryptedSourceDataSourceType::Sources(sources) => sources.into_iter().map(|source| {
                    Source {
                        url: source.file,
                        isM3U8: source.r#type == "hls",
                        quality: None,
                    }
                }).collect(),
                DecryptedSourceDataSourceType::Encrypted(_) => vec![],
            },
        })
    }

    // TODO: Improve this AI slop
    async fn get_client_key(&self, xrax: &str) -> EnmaResult<String> {
        let url = format!("https://megacloud.blog/embed-2/v3/e-1/{}", xrax);
        let resp = self
            .client
            .get(&url)
            .header(REFERER, HeaderValue::from_static("https://hianime.to/"))
            .send()
            .await
            .map_err(|e| EnmaError::src_fetch_error("megacloud:get_client_key", Some(e.to_string()), None))?;

        let text = resp
            .text()
            .await
            .map_err(|e| EnmaError::src_fetch_error("megacloud:get_client_key", Some(e.to_string()), None))?;

        // candidate regexes (in same order as the TS code)
        let regexes = vec![
            Regex::new(r#"<meta name="_gg_fb" content="[A-Za-z0-9]+">"#).map_err(|e| EnmaError::src_fetch_error("megacloud:get_client_key", Some(e.to_string()), None))?,
            Regex::new(r#"<!--\s+_is_th:[0-9a-zA-Z]+\s+-->"#).map_err(|e| EnmaError::src_fetch_error("megacloud:get_client_key", Some(e.to_string()), None))?,
            Regex::new(r#"<script>window._lk_db\s*=\s*\{[^\}]*\};</script>"#).map_err(|e| EnmaError::src_fetch_error("megacloud:get_client_key", Some(e.to_string()), None))?,
            Regex::new(r#"<div\s+data-dpi="[0-9a-zA-Z]+"\s+.*></div>"#).map_err(|e| EnmaError::src_fetch_error("megacloud:get_client_key", Some(e.to_string()), None))?,
            Regex::new(r#"<script nonce="[0-9a-zA-Z]+">"#).map_err(|e| EnmaError::src_fetch_error("megacloud:get_client_key", Some(e.to_string()), None))?,
            Regex::new(r#"<script>window._xy_ws\s*=\s*['"`][0-9a-zA-Z]+['"`];</script>"#).map_err(|e| EnmaError::src_fetch_error("megacloud:get_client_key", Some(e.to_string()), None))?,
        ];

        let mut found_index: Option<usize> = None;
        let mut matched_str: Option<String> = None;

        for (i, re) in regexes.iter().enumerate() {
            if let Some(mat) = re.find(&text) {
                found_index = Some(i);
                matched_str = Some(mat.as_str().to_string());
                break;
            }
        }

        let idx = found_index.ok_or_else(|| {
            EnmaError::src_fetch_error("megacloud:get_client_key", Some(String::from("Failed extracting client key segment")), None)
        })?;

        let matched = matched_str.unwrap_or_default();

        // helper regexes
        let quoted_key = Regex::new(r#""([A-Za-z0-9]+)""#).map_err(|e| EnmaError::src_fetch_error("megacloud:get_client_key", Some(e.to_string()), None))?;
        let comment_key = Regex::new(r#":([A-Za-z0-9]+)\s"#).map_err(|e| EnmaError::src_fetch_error("megacloud:get_client_key", Some(e.to_string()), None))?;
        // lk_db parts
        let rx = Regex::new(r#"x:\s*["']([A-Za-z0-9]+)["']"#).map_err(|e| EnmaError::src_fetch_error("megacloud:get_client_key", Some(e.to_string()), None))?;
        let ry = Regex::new(r#"y:\s*["']([A-Za-z0-9]+)["']"#).map_err(|e| EnmaError::src_fetch_error("megacloud:get_client_key", Some(e.to_string()), None))?;
        let rz = Regex::new(r#"z:\s*["']([A-Za-z0-9]+)["']"#).map_err(|e| EnmaError::src_fetch_error("megacloud:get_client_key", Some(e.to_string()), None))?;

        let client_key = if idx == 2 {
            // lk_db: assemble x + y + z
            let p1 = rx.captures(&matched).and_then(|c| c.get(1).map(|m| m.as_str())).ok_or_else(|| EnmaError::src_fetch_error("megacloud:get_client_key", Some(String::from("Failed building client key (x)")), None))?;
            let p2 = ry.captures(&matched).and_then(|c| c.get(1).map(|m| m.as_str())).ok_or_else(|| EnmaError::src_fetch_error("megacloud:get_client_key", Some(String::from("Failed building client key (y)")), None))?;
            let p3 = rz.captures(&matched).and_then(|c| c.get(1).map(|m| m.as_str())).ok_or_else(|| EnmaError::src_fetch_error("megacloud:get_client_key", Some(String::from("Failed building client key (z)")), None))?;
            format!("{p1}{p2}{p3}")
        } else if idx == 1 {
            // comment form: extract the alphanumeric after the colon
            let cap = comment_key.captures(&matched).and_then(|c| c.get(1).map(|m| m.as_str())).ok_or_else(|| EnmaError::src_fetch_error("megacloud:get_client_key", Some(String::from("Failed extracting client key (comment)")), None))?;
            cap.to_string()
        } else {
            // other forms: quoted key or nonce attribute; extract alphanumeric inside quotes
            let cap = quoted_key.captures(&matched).and_then(|c| c.get(1).map(|m| m.as_str())).ok_or_else(|| EnmaError::src_fetch_error("megacloud:get_client_key", Some(String::from("Failed extracting client key")), None))?;
            cap.to_string()
        };

        Ok(client_key)
    }
}