//! Starting point for interacting with a lotide API

use reqwest::{Method, RequestBuilder};
use secrecy::Secret;
use serde::de::DeserializeOwned;

use crate::{
    api_models::{CommunityInfo, InstanceInfo, List, PostListPost},
    prelude::{ReqCommunities, ReqPosts},
};

/// Starting point for interacting with a lotide API
pub struct Client {
    client: reqwest::Client,
    instance_url: String,
    token: Option<Secret<String>>,
}

impl Client {
    /// Create a new [`Client`]
    ///
    /// `instance_url` should be the full base url to the instance's API,
    /// e.g. `https://lotide.example.com/api/unstable`
    pub fn new(instance_url: impl ToString) -> Self {
        Self {
            client: reqwest::Client::new(),
            instance_url: instance_url.to_string(),
            token: None,
        }
    }

    /// Get the internal [`reqwest::Client`] instance
    pub fn reqwest_client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the internal [`reqwest::Client`] instance mutably
    pub fn reqwest_client_mut(&mut self) -> &mut reqwest::Client {
        &mut self.client
    }

    /// Get the stored instance URL
    ///
    /// This is the URL prepended to every request
    pub fn instance_url(&self) -> &str {
        &self.instance_url
    }

    /// Set the bearer token to be used with requests
    pub fn set_token(&mut self, token: impl ToString) {
        self.token = Some(Secret::new(token.to_string()));
    }

    /// Does the [`Client`] have a token set?
    ///
    /// Note that it is not possible to retrieve the actual token.
    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    /// Make a request to the instance for information about itself
    pub async fn instance_info(&self) -> reqwest::Result<InstanceInfo> {
        self.request(Method::GET, "instance").await
    }

    /// List communities on the instance
    pub async fn communities<'a>(
        &self,
        req: &ReqCommunities<'a>,
    ) -> reqwest::Result<List<CommunityInfo>> {
        self.request_with(Method::GET, "communities", |b| b.query(req))
            .await
    }

    /// List posts on the instance
    pub async fn posts<'a>(&self, req: &ReqPosts<'a>) -> reqwest::Result<List<PostListPost>> {
        self.request_with(Method::GET, "posts", |b| b.query(req))
            .await
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
        use secrecy::ExposeSecret;

        let builder = self
            .client
            .request(method, &format!("{}/{}", self.instance_url, subpath));

        let builder = f(builder);

        let builder = match &self.token {
            Some(token) => builder.bearer_auth(token.expose_secret()),
            None => builder,
        };

        builder.send().await?.json().await
    }
}
