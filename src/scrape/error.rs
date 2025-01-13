use std::fmt::{Display, Formatter};

/// A scraping error.
#[derive(Debug)]
pub enum Error {
    /// An invalid selection.
    InvalidSelection { selection: String, message: String },

    /// Expected a single value but got multiple.
    ExpectedOneGotMultiple { selection: String },

    /// Expected a single value but got none.
    ExpectedOneGotNone { selection: String },

    /// Expected a single value or no values but got multiple.
    ExpectedOptionalGotMultiple { selection: String },
}

impl Display for Error {
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
        }
    }
}

impl std::error::Error for Error {}
