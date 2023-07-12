use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;

use crate::api_models::InstanceInfo;

/// Starting point for interacting with a lotide API
pub struct Ctx {
    client: reqwest::Client,
    instance_url: String,
    token: Option<Box<str>>,
}

impl Ctx {
    /// Create a new [`Ctx`]
    ///
    /// `instance_url` should be the full base url to the instance's API,
    /// e.g. `https://lotide.example.com/api/unstable`
    pub fn new(instance_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            instance_url,
            token: None,
        }
    }

    /// Get the stored instance URL
    ///
    /// This is the URL prepended to every request
    pub fn instance_url(&self) -> &str {
        &self.instance_url
    }

    /// Set the bearer token to be used with requests
    pub fn set_token(&mut self, token: impl Into<Box<str>>) {
        self.token = Some(token.into());
    }

    /// Does the [`Ctx`] have a token set?
    ///
    /// Note that it is not possible to retrieve the actual token.
    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    /// Make a request to the instance for information about itself
    pub async fn instance_info(&self) -> Result<InstanceInfo, reqwest::Error> {
        self.request(Method::GET, "instance").await
    }

    /// Make a request to the instance
    pub async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        subpath: &str,
    ) -> Result<T, reqwest::Error> {
        self.request_with(method, subpath, |b| b).await
    }

    /// Like [`Ctx::request`](`#method.request`), but allows you to modify the [`RequestBuilder`] before sending
    pub async fn request_with<T: DeserializeOwned>(
        &self,
        method: Method,
        subpath: &str,
        f: impl Fn(RequestBuilder) -> RequestBuilder,
    ) -> Result<T, reqwest::Error> {
        let mut builder = self
            .client
            .request(method, &format!("{}/{}", self.instance_url, subpath));

        if let Some(token) = &self.token {
            builder = builder.bearer_auth(token);
        }

        let builder = f(builder);

        builder.send().await?.json().await
    }
}
