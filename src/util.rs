use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT},
    Client,
};

// const ACCEPT_ENCODING_HEADER: &str = "gzip, deflate, br";
pub const USER_AGENT_HEADER: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";
pub const ACCEPT_HEADER: &str = "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9";

pub fn new_http_client(req_headers: Option<HeaderMap>) -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_HEADER));
    headers.insert(ACCEPT, HeaderValue::from_static(ACCEPT_HEADER));

    if let Some(r_headers) = req_headers {
        for (k, v) in r_headers {
            if let Some(name) = k {
                headers.insert(name, v);
            }
        }
    }

    return Client::builder()
        .default_headers(headers)
        .build()
        .expect("Could not initialize HTTP client");
}
