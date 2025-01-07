#![allow(dead_code)] // TODO: remove this

use crate::error::{EnmaError, EnmaResult};
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, USER_AGENT},
    Client, StatusCode,
};
use serde::de::DeserializeOwned;
use urlencoding::{decode, encode};

pub enum EnmaUtils {
    AcceptHeader,
    UserAgentHeader,
    AcceptEncodingHeader,
    XRequestedWithHeader,
}

impl EnmaUtils {
    pub fn value(&self) -> &'static str {
        match self {
            EnmaUtils::AcceptHeader => "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9",
            EnmaUtils::UserAgentHeader => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",  
            EnmaUtils::AcceptEncodingHeader => "gzip, deflate, br",
            EnmaUtils::XRequestedWithHeader => "XMLHttpRequest",
        }
    }

    pub fn new_http_client(req_headers: Option<HeaderMap>) -> Client {
        let mut headers: HeaderMap = [
            (
                USER_AGENT,
                HeaderValue::from_static(EnmaUtils::UserAgentHeader.value()),
            ),
            (
                ACCEPT,
                HeaderValue::from_static(EnmaUtils::AcceptHeader.value()),
            ),
            (
                ACCEPT_ENCODING,
                HeaderValue::from_static(EnmaUtils::AcceptEncodingHeader.value()),
            ),
        ]
        .into_iter()
        .collect();

        if let Some(r_headers) = req_headers {
            for (k, v) in r_headers {
                if let Some(name) = k {
                    headers.insert(name, v);
                }
            }
        }

        return Client::builder()
            .gzip(true)
            .brotli(true)
            .deflate(true)
            .default_headers(headers)
            .build()
            .expect("Could not initialize HTTP client");
    }

    pub fn encode_uri_component(
        provider_parser: &'static str,
        raw_string: String,
    ) -> EnmaResult<String> {
        let encoded_str = encode(raw_string.trim()).into_owned();

        if encoded_str.is_empty() {
            return Err(EnmaError::parsing_error(
                provider_parser,
                Some(String::from("encoded string is empty")),
                Some(StatusCode::BAD_REQUEST),
            ));
        }

        Ok(encoded_str)
    }

    pub fn decode_uri_component(
        provider_parser: &'static str,
        raw_string: String,
    ) -> EnmaResult<String> {
        let decoded_str = decode(raw_string.trim())
            .map_err(|_| {
                EnmaError::parsing_error(
                    provider_parser,
                    Some(String::from("failed to parse raw string")),
                    Some(StatusCode::BAD_REQUEST),
                )
            })?
            .into_owned();

        if decoded_str.is_empty() {
            return Err(EnmaError::parsing_error(
                provider_parser,
                Some(String::from("decoded string is empty")),
                Some(StatusCode::BAD_REQUEST),
            ));
        }

        Ok(decoded_str)
    }
}

pub trait EnmaClient {
    async fn get_html(
        &self,
        url: String,
        headers: Option<HeaderMap>,
        provider_parser: &'static str,
    ) -> EnmaResult<String>;

    async fn get_json<T: DeserializeOwned>(
        &self,
        url: String,
        headers: Option<HeaderMap>,
        provider_parser: &'static str,
    ) -> EnmaResult<T>;
}

impl EnmaClient for Client {
    async fn get_html(
        &self,
        url: String,
        headers: Option<HeaderMap>,
        provider_parser: &'static str,
    ) -> EnmaResult<String> {
        let response = match self
            .get(url)
            .headers(headers.unwrap_or_default())
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(_) => return Err(EnmaError::src_fetch_error(provider_parser, None, None)),
        };

        let html = response
            .text()
            .await
            .map_err(|_| EnmaError::src_parse_error(provider_parser, None, None))?;

        if html.is_empty() {
            return Err(EnmaError::src_parse_error(provider_parser, None, None));
        }

        return Ok(html);
    }

    async fn get_json<T: DeserializeOwned>(
        &self,
        url: String,
        headers: Option<HeaderMap>,
        provider_parser: &'static str,
    ) -> EnmaResult<T> {
        let response = match self
            .get(url)
            .headers(headers.unwrap_or_default())
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(_) => return Err(EnmaError::src_fetch_error(provider_parser, None, None)),
        };

        let data = response
            .json::<T>()
            .await
            .map_err(|_| EnmaError::src_parse_error(provider_parser, None, None))?;

        Ok(data)
    }
}
