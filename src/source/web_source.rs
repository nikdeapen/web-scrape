use bytes::Bytes;
use reqwest::blocking::{Client, Request, Response};
use reqwest::header::HeaderMap;
use reqwest::{Method, StatusCode, Url};
use web_url::WebUrl;

use crate::cache::WebCache;
use crate::error::Error;

/// Responsible for sourcing data from the web.
#[derive(Clone, Debug)]
pub struct WebSource {
    client: Client,
    headers: HeaderMap,
    cache: WebCache,
}

impl WebSource {
    //! Construction

    /// Creates a new web source.
    pub fn new(headers: HeaderMap, cache: WebCache) -> Self {
        Self {
            client: Client::default(),
            headers,
            cache,
        }
    }
}

impl WebSource {
    //! Get

    /// Gets the text content from the `url`.
    pub fn get(&self, url: &WebUrl) -> Result<String, Error> {
        Ok(String::from_utf8(self.get_data(url)?)?)
    }

    /// Gets the data content from the `url`.
    pub fn get_data(&self, url: &WebUrl) -> Result<Vec<u8>, Error> {
        if let Some(cached) = self.cache.read(url)? {
            return Ok(cached);
        }
        let data: Vec<u8> = self.download(Method::GET, url)?;
        self.cache.write(url, data.as_slice())?;
        Ok(data)
    }
}

impl WebSource {
    //! Download

    /// Downloads the data from the `url`
    fn download(&self, method: Method, url: &WebUrl) -> Result<Vec<u8>, Error> {
        let response: Response = self.client.execute(self.create_request(method, url)?)?;
        match response.status() {
            StatusCode::OK | StatusCode::NO_CONTENT => {
                let content: Bytes = response.bytes()?;
                Ok(content.to_vec())
            }
            status => Err(Error::InvalidResponseStatus(status)),
        }
    }

    /// Creates the request.
    fn create_request(&self, method: Method, url: &WebUrl) -> Result<Request, Error> {
        if url.scheme().as_str() != "http" && url.scheme().as_str() != "https" {
            Err(Error::InvalidURL {
                url: url.clone(),
                error_message: "the scheme must be 'http' or 'https'".to_string(),
            })
        } else if url.fragment().is_some() {
            Err(Error::InvalidURL {
                url: url.clone(),
                error_message: "the fragment must be empty".to_string(),
            })
        } else {
            let mut request: Request = Request::new(
                method,
                Url::parse(url.as_str()).map_err(|e| Error::InvalidURL {
                    url: url.clone(),
                    error_message: e.to_string(),
                })?,
            );
            for (name, value) in &self.headers {
                request.headers_mut().insert(name, value.clone());
            }
            Ok(request)
        }
    }
}
