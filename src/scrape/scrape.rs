use crate::scrape::{Error, Scraper};

/// An element that can scrape itself from the web.
pub trait Scrape: Sized {
    /// Scrapes an instance of the element from the scraper `s`.
    fn scrape(s: Scraper) -> Result<Self, Error>;
}
