use async_trait::async_trait;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use scraper::Html;
use sha256::digest;
use tokio::fs;
use web_url::WebUrl;

use crate::extract::Error::Other;
use crate::extract::{Error, Extract, Extractor, Parser, Webpage};

/// An extractor that uses a local cache. (this is mainly for dev/debug)
pub struct CacheExtractor<T> {
    cache_folder: String,
    extractor: Box<dyn Extractor<T> + Send + Sync>,
    parser: Box<dyn Parser<T> + Send + Sync>,
}

impl<T> CacheExtractor<T> {
    //! Construction

    /// Creates a new cache extractor.
    pub fn new<S, E, P>(cache_folder: S, extractor: E, parser: P) -> Self
    where
        S: Into<String>,
        E: 'static + Extractor<T> + Send + Sync,
        P: 'static + Parser<T> + Send + Sync,
    {
        Self {
            cache_folder: cache_folder.into(),
            extractor: Box::new(extractor),
            parser: Box::new(parser),
        }
    }
}

impl<T> CacheExtractor<T> {
    //! Caching

    /// Gets the cache file path.
    fn cache_file_path(&self, url: &WebUrl) -> String {
        let hash: String = digest(url.url());
        let mut path: String = String::default();
        path.push_str(self.cache_folder.as_str());
        path.push_str(hash.as_str());
        path.push_str(".html");
        path
    }

    /// Gets the extract from the cache. Returns None if the url was not cached.
    async fn get(&self, url: &WebUrl) -> Result<Option<Extract<T>>, Error> {
        let path: String = self.cache_file_path(url);
        if fs::metadata(path.as_str()).await.is_ok() {
            let content: String = fs::read_to_string(path.as_str())
                .await
                .map_err(|e| Other(e.to_string()))?;
            let webpage: Webpage = Webpage {
                url: url.clone(),
                status: StatusCode::OK,
                headers: HeaderMap::default(),
                content,
            };
            let html: Html = Html::parse_document(webpage.content.as_str());
            let data: T = self.parser.parse(&webpage, &html)?;
            println!("reading from cache: {}", url);
            Ok(Some(Extract { webpage, data }))
        } else {
            Ok(None)
        }
    }

    /// Sets the data for the url in the cache.
    async fn set(&self, url: &WebUrl, content: &str) -> Result<(), Error> {
        let path: String = self.cache_file_path(url);
        if let Some(parent) = std::path::Path::new(path.as_str()).parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| Other(e.to_string()))?;
        }
        fs::write(path.as_str(), content)
            .await
            .map_err(|e| Other(e.to_string()))
    }
}

#[async_trait]
impl<T: Send + Sync> Extractor<T> for CacheExtractor<T> {
    async fn extract(&self, url: &WebUrl) -> Result<Extract<T>, Error> {
        if let Some(extract) = self.get(url).await? {
            Ok(extract)
        } else {
            let extract: Extract<T> = self.extractor.extract(url).await?;
            self.set(url, extract.webpage.content.as_str()).await?;
            Ok(extract)
        }
    }
}
