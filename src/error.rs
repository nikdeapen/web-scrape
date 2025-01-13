use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

use reqwest::StatusCode;
use web_url::WebUrl;

/// An error scraping data from the web.
#[derive(Debug)]
pub enum Error {
    /// An invalid URL string.
    InvalidURLString(web_url::parse::Error),

    /// An invalid URL.
    InvalidURL { url: WebUrl, error_message: String },

    /// A protocol error.
    Protocol(reqwest::Error),

    /// An invalid response status code.
    InvalidStatus(StatusCode),

    /// A storage error.
    Storage(file_storage::Error),

    /// The text was not UTF-8.
    InvalidText(FromUtf8Error),

    /// A scraping error.
    Scrape(crate::scrape::Error),

    /// An uncategorized error.
    Other(String),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Protocol(error)
    }
}

impl From<file_storage::Error> for Error {
    fn from(error: file_storage::Error) -> Self {
        Self::Storage(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Self::InvalidText(error)
    }
}

impl From<crate::scrape::Error> for Error {
    fn from(error: crate::scrape::Error) -> Self {
        Self::Scrape(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidURLString(error) => {
                write!(f, "invalid url string: {}", error)
            }
            Self::InvalidURL { url, error_message } => {
                write!(f, "invalid url '{}': {}", url, error_message)
            }
            Self::Protocol(error) => write!(f, "{}", error),
            Self::InvalidStatus(status) => write!(f, "invalid response status code: {}", status),
            Self::Storage(error) => write!(f, "{}", error),
            Self::InvalidText(error) => write!(f, "invalid text: {}", error),
            Self::Scrape(error) => write!(f, "{}", error),
            Self::Other(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for Error {}
