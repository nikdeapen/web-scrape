use async_trait::async_trait;
use web_url::WebUrl;

use crate::extract::{Error, Extract};

/// Responsible for extracting data from webpages.
#[async_trait]
pub trait Extractor<T> {
    /// Extracts data from the webpage.
    async fn extract(&self, url: &WebUrl) -> Result<Extract<T>, Error>;
}
