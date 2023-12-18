/// An error extracting data from a webpage.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Error {
    /// An error setting up data extraction.
    Setup(String),

    /// An error executing the initial request.
    Execution(String),

    /// An error downloading or waiting for content.
    Download(String),

    /// An unspecified error.
    Other(String),
}
