use bytes::Bytes;
use reqwest::blocking::{Client, Request, Response};
use reqwest::header::HeaderMap;
use reqwest::{Method, StatusCode, Url};
use web_url::WebUrl;

use crate::error::Error;

/// Responsible for sourcing data from the web.
#[derive(Clone, Debug)]
pub struct WebSource {
    client: Client,
    headers: HeaderMap,
}

impl WebSource {
    //! Download

    /// Downloads the data from the `url`
    pub fn download(&self, method: Method, url: &WebUrl) -> Result<Vec<u8>, Error> {
        let response: Response = self.client.execute(self.create_request(method, url)?)?;
        match response.status() {
            StatusCode::OK | StatusCode::NO_CONTENT => {
                let content: Bytes = response.bytes()?;
                Ok(content.to_vec())
            }
            status => Err(Error::InvalidStatus(status)),
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
