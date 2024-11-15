use std::fmt::{Display, Formatter};

use reqwest::StatusCode;
use web_url::WebUrl;

/// An error scraping data from the web.
#[derive(Debug)]
pub enum Error {
    /// An invalid URL.
    InvalidURL { url: WebUrl, error_message: String },

    /// A protocol error.
    Protocol(reqwest::Error),

    /// An invalid response status code.
    InvalidStatus(StatusCode),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Protocol(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidURL { url, error_message } => {
                write!(f, "invalid url '{}': {}", url, error_message)
            }
            Self::Protocol(error) => write!(f, "{}", error),
            Self::InvalidStatus(status) => write!(f, "invalid response status code: {}", status),
        }
    }
}

impl std::error::Error for Error {}
