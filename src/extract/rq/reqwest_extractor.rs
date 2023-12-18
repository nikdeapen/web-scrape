use async_trait::async_trait;
use reqwest::header::HeaderMap;
use reqwest::{Client, Request, Response, StatusCode};
use scraper::Html;
use web_url::WebUrl;

use crate::extract::Error::{Download, Execution, Setup};
use crate::extract::{Error, Extract, Extractor, Parser, Webpage};

/// Uses the reqwest crate to extract data from webpages.
pub struct ReqwestExtractor<T> {
    client: Client,
    parser: Box<dyn Parser<T> + Send + Sync>,
}

impl<T> ReqwestExtractor<T> {
    //! Construction

    /// Creates a new reqwest extractor.
    pub fn new<P>(client: Client, parser: P) -> Self
    where
        P: 'static + Parser<T> + Send + Sync,
    {
        Self {
            client,
            parser: Box::new(parser),
        }
    }
}

impl<T: Send + Sync> ReqwestExtractor<T> {
    //! Requests

    /// Creates the request.
    fn create_request(&self, url: &WebUrl) -> Result<Request, Error> {
        self.client
            .get(url.url())
            .build()
            .map_err(|e| Setup(e.to_string()))
    }

    /// Executes the request.
    async fn execute_request(&self, request: Request) -> Result<Response, Error> {
        self.client
            .execute(request)
            .await
            .map_err(|e| Execution(e.to_string()))
    }

    /// Downloads the content.
    async fn download(&self, url: &WebUrl, response: Response) -> Result<Webpage, Error> {
        let status: StatusCode = response.status();
        let headers: HeaderMap = response.headers().clone();
        let content: String = response.text().await.map_err(|e| Download(e.to_string()))?;
        Ok(Webpage {
            url: url.clone(),
            status,
            headers,
            content,
        })
    }
}

#[async_trait]
impl<T: Send + Sync> Extractor<T> for ReqwestExtractor<T> {
    async fn extract(&self, url: &WebUrl) -> Result<Extract<T>, Error> {
        let request: Request = self.create_request(url)?;
        let response: Response = self.execute_request(request).await?;
        let webpage: Webpage = self.download(url, response).await?;
        let html: Html = Html::parse_document(webpage.content.as_str());
        let data: T = self.parser.parse(&webpage, &html)?;
        Ok(Extract { webpage, data })
    }
}
