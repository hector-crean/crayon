pub mod reqwest;

use std::future::Future;
use serde::de::DeserializeOwned;

use crate::CrayonServerError;

pub trait HttpClient<'a>: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static + Into<CrayonServerError>;
    type Url: Into<url::Url> + From<url::Url>;
    type Body;
    type Future<T: 'a>: Future<Output = Result<T, Self::Error>> + Send + 'a;

    
    fn get<R>(&'a self, url: Self::Url) -> Self::Future<R>
    where
        R: DeserializeOwned + 'a;

    fn post<T, R>(&'a self, url: Self::Url, body: T) -> Self::Future<R>
    where
        T: Into<Self::Body>,
        R: DeserializeOwned + 'a;

    fn put<T, R>(&'a self, url: Self::Url, body: T) -> Self::Future<R>
    where
        T: Into<Self::Body>,
        R: DeserializeOwned + 'a;

    fn patch<T, R>(&'a self, url: Self::Url, body: T) -> Self::Future<R>
    where
        T: Into<Self::Body>,
        R: DeserializeOwned + 'a;

    fn delete<R>(&'a self, url: Self::Url) -> Self::Future<R>
    where
        R: DeserializeOwned + 'a;

    fn head<R>(&'a self, url: Self::Url) -> Self::Future<R>
    where
        R: DeserializeOwned + 'a;

    fn options<R>(&'a self, url: Self::Url) -> Self::Future<R>
    where
        R: DeserializeOwned + 'a;
}