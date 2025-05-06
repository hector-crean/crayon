use std::future::Future;
use std::pin::Pin;
use serde::de::DeserializeOwned;

use super::HttpClient;


#[derive(Clone)]
pub struct ReqwestClient(reqwest::Client);

impl Default for ReqwestClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ReqwestClient {
    pub fn new() -> Self {
        Self(reqwest::Client::new())
    }
}

impl From<reqwest::Client> for ReqwestClient {
    fn from(client: reqwest::Client) -> Self {
        Self(client)
    }
}

impl<'a> HttpClient<'a> for ReqwestClient {
    type Error = reqwest::Error;
    type Url = reqwest::Url;
    type Body = reqwest::Body;
    type Future<T: 'a> = Pin<Box<dyn Future<Output = Result<T, Self::Error>> + Send + 'a>>;

    fn get<R>(&'a self, url: Self::Url) -> Self::Future<R>
    where
        R: DeserializeOwned + 'a,
    {
        Box::pin(async move {
            self.0
                .get(url)
                .send()
                .await?
                .json::<R>()
                .await
        })
    }

    fn post<T, R>(&'a self, url: Self::Url, body: T) -> Self::Future<R>
    where
        T: Into<Self::Body>,
        R: DeserializeOwned + 'a,
    {
        let body = body.into();
        Box::pin(async move {
            self.0
                .post(url)
                .body(body)
                .send()
                .await?
                .json::<R>()
                .await
        })
    }

    fn put<T, R>(&'a self, url: Self::Url, body: T) -> Self::Future<R>
    where
        T: Into<Self::Body>,
        R: DeserializeOwned + 'a,
    {
        let body = body.into();
        Box::pin(async move {
            self.0
                .put(url)
                .body(body)
                .send()
                .await?
                .json::<R>()
                .await
        })
    }

    fn patch<T, R>(&'a self, url: Self::Url, body: T) -> Self::Future<R>
    where
        T: Into<Self::Body>,
        R: DeserializeOwned + 'a,
    {
        let body = body.into();
        Box::pin(async move {
            self.0
                .patch(url)
                .body(body)
                .send()
                .await?
                .json::<R>()
                .await
        })
    }

    fn delete<R>(&'a self, url: Self::Url) -> Self::Future<R>
    where
        R: DeserializeOwned + 'a,
    {
        Box::pin(async move {
            self.0
                .delete(url)
                .send()
                .await?
                .json::<R>()
                .await
        })
    }

    fn head<R>(&'a self, url: Self::Url) -> Self::Future<R>
    where
        R: DeserializeOwned + 'a,
    {
        Box::pin(async move {
            self.0
                .head(url)
                .send()
                .await?
                .json::<R>()
                .await
        })
    }

    fn options<R>(&'a self, url: Self::Url) -> Self::Future<R>
    where
        R: DeserializeOwned + 'a,
    {
        Box::pin(async move {
            self.0
                .request(reqwest::Method::OPTIONS, url)
                .send()
                .await?
                .json::<R>()
                .await
        })
    }
}