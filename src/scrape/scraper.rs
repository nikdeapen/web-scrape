use scraper::element_ref::Select;
use scraper::{ElementRef, Selector};

use crate::scrape::Error;
use crate::scrape::Error::*;

/// Responsible for scraping data from elements.
#[derive(Copy, Clone, Debug)]
pub struct Scraper<'a> {
    element: ElementRef<'a>,
}

impl<'a> From<ElementRef<'a>> for Scraper<'a> {
    fn from(element: ElementRef<'a>) -> Self {
        Self { element }
    }
}

impl<'a> Scraper<'a> {
    //! Properties

    /// Gets the element.
    pub fn element(&self) -> ElementRef {
        self.element
    }
}

impl<'a> Scraper<'a> {
    //! Utils

    /// Creates the `Selector` for the `selection`.
    fn selector(selection: &str) -> Result<Selector, Error> {
        Selector::parse(selection).map_err(|e| InvalidSelection {
            selection: selection.to_string(),
            message: e.to_string(),
        })
    }
}

impl<'a> Scraper<'a> {
    //! All

    /// Scrapes all the instances of the `selection`.
    pub fn all<T, F>(&self, selection: &str, scrape: F) -> Result<Vec<T>, Error>
    where
        F: Fn(Scraper) -> Result<T, Error>,
    {
        let selector: Selector = Self::selector(selection)?;
        let mut result: Vec<T> = Vec::default();
        for element in self.element.select(&selector) {
            let scraper: Scraper = Scraper::from(element);
            let element: T = scrape(scraper)?;
            result.push(element)
        }
        Ok(result)
    }

    /// Scrapes all the successful instances of the `selection`.
    pub fn all_flat<T, F>(&self, selection: &str, scrape: F) -> Result<Vec<T>, Error>
    where
        F: Fn(Scraper) -> Result<Option<T>, Error>,
    {
        let selector: Selector = Self::selector(selection)?;
        let mut result: Vec<T> = Vec::default();
        for element in self.element.select(&selector) {
            let scraper: Scraper = Scraper::from(element);
            if let Some(element) = scrape(scraper)? {
                result.push(element)
            }
        }
        Ok(result)
    }
}

impl<'a> Scraper<'a> {
    //! Only

    /// Scrapes the only instance of the `selection`.
    pub fn only<T, F>(&self, selection: &str, scrape: F) -> Result<T, Error>
    where
        F: Fn(Scraper) -> Result<T, Error>,
    {
        let selector: Selector = Self::selector(selection)?;
        let mut select: Select = self.element.select(&selector);
        if let Some(first) = select.next() {
            let first: T = scrape(first.into())?;
            if select.next().is_some() {
                Err(ExpectedOneGotMultiple {
                    selection: selection.to_string(),
                })
            } else {
                Ok(first)
            }
        } else {
            Err(ExpectedOneGotNone {
                selection: selection.to_string(),
            })
        }
    }

    /// Scrapes the only instance of the `selection` attribute.
    pub fn only_att(&self, selection: &str, att: &str) -> Result<String, Error> {
        self.only(selection, |s| {
            if let Some(att) = s.element.attr(att) {
                Ok(att.to_string())
            } else {
                Err(ExpectedOneGotNone {
                    selection: selection.to_string(),
                })
            }
        })
    }

    /// Scrapes the only instance of the `selection` text.
    pub fn only_text(&self, selection: &str) -> Result<String, Error> {
        self.only(selection, |s| Ok(s.element.text().collect()))
    }
}

impl<'a> Scraper<'a> {
    //! Optional

    /// Scrapes the optional instance of the `selection`.
    pub fn optional<T, F>(&self, selection: &str, scrape: F) -> Result<Option<T>, Error>
    where
        F: Fn(Scraper) -> Result<T, Error>,
    {
        let selector: Selector = Self::selector(selection)?;
        let mut select: Select = self.element.select(&selector);
        if let Some(first) = select.next() {
            let first: T = scrape(first.into())?;
            if select.next().is_some() {
                Err(ExpectedOptionalGotMultiple {
                    selection: selection.to_string(),
                })
            } else {
                Ok(Some(first))
            }
        } else {
            Ok(None)
        }
    }

    /// Scrapes the optional instance of the `selection` text.
    pub fn optional_text(&self, selection: &str) -> Result<Option<String>, Error> {
        self.optional(selection, |s| Ok(s.element().text().collect()))
    }
}