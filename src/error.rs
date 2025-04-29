use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

use clerr::Report;
use reqwest::StatusCode;
use web_url::WebUrl;

use crate::scrape::ScrapeError;

/// An error scraping data from the web.
#[derive(Debug)]
pub enum Error {
    /// A storage error.
    Storage(file_storage::Error),

    /// The URL was invalid.
    InvalidURL { url: WebUrl, error_message: String },

    /// The URL string was invalid.
    InvalidURLString {
        url: String,
        error: web_url::parse::Error,
    },

    /// An error converting the web content to UTF-8.
    InvalidText(FromUtf8Error),

    /// A protocol error.
    Protocol(reqwest::Error),

    /// An invalid response status was received.
    InvalidResponseStatus(StatusCode),

    /// A scraping error.
    Scrape(ScrapeError),

    /// An error report.
    Other(Report),
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

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Protocol(error)
    }
}

impl From<ScrapeError> for Error {
    fn from(error: ScrapeError) -> Self {
        Self::Scrape(error)
    }
}

impl From<Report> for Error {
    fn from(report: Report) -> Self {
        Self::Other(report)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
