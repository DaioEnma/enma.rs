use crate::error::{EnmaError, EnmaResult};
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, USER_AGENT},
    Client,
};

pub enum EnmaUtils {
    AcceptHeader,
    UserAgentHeader,
    AcceptEncodingHeader,
}

impl EnmaUtils {
    pub fn new_http_client(req_headers: Option<HeaderMap>) -> Client {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(EnmaUtils::UserAgentHeader.value()),
        );
        headers.insert(
            ACCEPT,
            HeaderValue::from_static(EnmaUtils::AcceptHeader.value()),
        );
        headers.insert(
            ACCEPT_ENCODING,
            HeaderValue::from_static(EnmaUtils::AcceptEncodingHeader.value()),
        );

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

    pub fn value(&self) -> &'static str {
        match self {
            Self::AcceptHeader => "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9",
            Self::UserAgentHeader => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",  
            Self::AcceptEncodingHeader => "gzip, deflate, br",
        }
    }
}

pub trait HtmlLoader {
    async fn get_html(
        &self,
        url: &'static str,
        headers: Option<HeaderMap>,
        provider_parser: &'static str,
    ) -> EnmaResult<String>;
}

impl HtmlLoader for Client {
    async fn get_html(
        &self,
        url: &'static str,
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

        return Ok(html);
    }
}
