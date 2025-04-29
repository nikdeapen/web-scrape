use std::fmt::{Display, Formatter};

/// A scraping error.
#[derive(Debug)]
pub enum ScrapeError {
    /// An invalid selection.
    InvalidSelection { selection: String, message: String },

    /// Expected a single value but got multiple.
    ExpectedOneGotMultiple { selection: String },

    /// Expected a single value but got none.
    ExpectedOneGotNone { selection: String },

    /// Expected a single value or no values but got multiple.
    ExpectedOptionalGotMultiple { selection: String },

    /// Another error.
    Other(String),
}

impl<S: Into<String>> From<S> for ScrapeError {
    fn from(message: S) -> Self {
        Self::Other(message.into())
    }
}

impl Display for ScrapeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSelection { selection, message } => {
                write!(f, "invalid selection '{}': {}", selection, message)
            }
            Self::ExpectedOneGotMultiple { selection } => {
                write!(f, "expected one, got multiple: {}", selection)
            }
            Self::ExpectedOneGotNone { selection } => {
                write!(f, "expected one, got none: {}", selection)
            }
            Self::ExpectedOptionalGotMultiple { selection } => {
                write!(f, "expected one or none, got multiple: {}", selection)
            }
            Self::Other(message) => {
                write!(f, "error scraping page: {}", message)
            }
        }
    }
}

impl std::error::Error for ScrapeError {}
