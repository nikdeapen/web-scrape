use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

use clerr::Report;

/// An error scraping data from the web.
#[derive(Debug)]
pub enum Error {
    /// A storage error.
    Storage(file_storage::Error),

    /// An error converting the web content to UTF-8.
    InvalidText(FromUtf8Error),

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
