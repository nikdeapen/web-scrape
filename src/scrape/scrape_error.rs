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

    /// An uncategorized error.
    Other(String),
}

impl From<String> for ScrapeError {
    fn from(message: String) -> Self {
        Self::Other(message)
    }
}

impl From<&str> for ScrapeError {
    fn from(message: &str) -> Self {
        Self::Other(message.to_string())
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
