use reqwest::{StatusCode, Url};

use super::HttpRequest;

pub struct Client {
    inner: reqwest::Client,
    url: Url,
}

#[derive(Debug)]
pub enum Error {
    Gemini(crate::error::Error),
    Reqwest(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl From<crate::error::Error> for Error {
    fn from(e: crate::error::Error) -> Self {
        Self::Gemini(e)
    }
}

impl Client {
    pub fn new(url: Url) -> Self {
        Self {
            inner: reqwest::Client::default(),
            url,
        }
    }

    pub async fn request<R: HttpRequest>(&self, req: R) -> Result<R::Response, Error> {
        let resp = self
            .inner
            .request(R::METHOD, req.url(self.url.clone()))
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok(resp.json::<R::Response>().await?),
            _ => Err(resp.json::<crate::error::Error>().await?.into()),
        }
    }
}
