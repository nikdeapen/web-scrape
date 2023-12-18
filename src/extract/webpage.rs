use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use web_url::WebUrl;

/// A downloaded webpage from a URL.
#[derive(Clone, Debug)]
pub struct Webpage {
    pub url: WebUrl,
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub content: String,
}
