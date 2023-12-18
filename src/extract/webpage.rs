use std::fmt::{Debug, Formatter};

use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use web_url::WebUrl;

/// A downloaded webpage from a URL.
#[derive(Clone)]
pub struct Webpage {
    pub url: WebUrl,
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub content: String,
}

impl Debug for Webpage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ url={}, status={}, headers={:?}, content-len={}",
            self.url.url(),
            self.status,
            self.headers,
            self.content.len()
        )
    }
}
