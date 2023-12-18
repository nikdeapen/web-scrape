use scraper::Html;

use crate::extract::error::Error;
use crate::extract::webpage::Webpage;

/// Responsible for parsing webpage content.
pub trait Parser<T> {
    /// Parses data from the webpage.
    fn parse(&self, webpage: &Webpage, html: &Html) -> Result<T, Error>;
}
