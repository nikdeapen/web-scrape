use crate::scrape::ScrapeError;
use clerr::Report;
use reqwest::StatusCode;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;
use web_url::WebUrl;

/// An error scraping data from the web.
#[derive(Debug)]
pub enum Error {
    /// An uncategorized error.
    Other(Report),

    /// The web data was not properly UTF-8 encoded.
    InvalidString(FromUtf8Error),

    /// The response status code was invalid.
    InvalidResponseStatus(StatusCode),

    /// The URL was invalid.
    InvalidURL { url: WebUrl, error_message: String },

    /// There was an error reading or writing the cache.
    Cache(file_storage::Error),

    /// There was an error sourcing the data.
    Source(reqwest::Error),

    /// There was an error scraping the data.
    Scrape(ScrapeError),
}

impl From<Report> for Error {
    fn from(report: Report) -> Self {
        Self::Other(report)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Self::InvalidString(error)
    }
}

impl From<file_storage::Error> for Error {
    fn from(error: file_storage::Error) -> Self {
        Self::Cache(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Source(error)
    }
}

impl From<ScrapeError> for Error {
    fn from(error: ScrapeError) -> Self {
        Self::Scrape(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other(report) => write!(f, "{}", report),
            Self::InvalidString(error) => write!(f, "invalid utf-8 string: {}", error),
            Self::InvalidResponseStatus(status) => {
                write!(f, "invalid response status: {}", status)
            }
            Self::InvalidURL { url, error_message } => {
                write!(f, "invalid url '{}': {}", url, error_message)
            }
            Self::Cache(error) => write!(f, "cache error: {}", error),
            Self::Source(error) => write!(f, "source error: {}", error),
            Self::Scrape(error) => write!(f, "scrape error: {}", error),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidString(error) => Some(error),
            Self::Cache(error) => Some(error),
            Self::Source(error) => Some(error),
            Self::Scrape(error) => Some(error),
            _ => None,
        }
    }
}
