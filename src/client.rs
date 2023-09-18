//! Starting point for interacting with a lotide API

use reqwest::{Method, RequestBuilder};
use secrecy::Secret;
use serde::de::DeserializeOwned;

use crate::{
    api_models::{CommunityInfo, InstanceInfo, List, LoginInfo, PostListPost},
    prelude::{ReqCommunities, ReqPosts, ReqRegister},
};

/// Starting point for interacting with a lotide API
pub struct Client {
    client: reqwest::Client,
    instance_url: String,
    lang: Option<String>,
    token: Option<Secret<String>>,
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        // PartialEq is useful for things like Yew
        let tokens_are_equal = if let (Some(a), Some(b)) = (&self.token, &other.token) {
            use secrecy::ExposeSecret;
            a.expose_secret() == b.expose_secret()
        } else {
            // TODO: Address the potential security concern with comparing tokens like this
            self.token.is_none() && other.token.is_none()
        };
        self.instance_url == other.instance_url && tokens_are_equal
    }
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
            lang: None,
            token: None,
        }
    }

    /// Set the language
    pub fn set_lang(&mut self, lang: impl ToString) {
        self.lang = Some(lang.to_string());
    }

    /// Set the language
    pub fn with_lang(mut self, lang: impl ToString) -> Self {
        self.set_lang(lang);
        self
    }

    /// Get the language
    pub fn lang(&self) -> Option<&str> {
        self.lang.as_deref()
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

    /// Log in to the service
    pub async fn login(
        &self,
        username: impl ToString,
        password: impl ToString,
    ) -> reqwest::Result<(Client, LoginInfo)> {
        #[derive(serde::Deserialize)]
        struct LoginResponse {
            token: Secret<String>,
            #[serde(flatten)]
            user: LoginInfo,
        }

        let LoginResponse { token, user } = self
            .request_with(Method::POST, "logins", |b| {
                b.json(&serde_json::json!({
                    "username": username.to_string(),
                    "password": password.to_string(),
                }))
            })
            .await?;

        let new_self = Self {
            client: self.client.clone(),
            instance_url: self.instance_url.clone(),
            lang: self.lang.clone(),
            token: Some(token),
        };

        Ok((new_self, user))
    }

    /// Fetch current login state
    pub async fn current_login(&self) -> reqwest::Result<LoginInfo> {
        self.request(Method::GET, "logins/~current").await
    }

    /// Log out
    pub async fn log_out(&self) -> reqwest::Result<()> {
        self.request(Method::DELETE, "logins/~current").await
    }

    /// Register a new account
    pub async fn register<'a>(
        &self,
        req: &ReqRegister<'a>,
    ) -> reqwest::Result<(Option<Client>, LoginInfo)> {
        #[derive(serde::Deserialize)]
        struct RegisterResponse {
            token: Option<Secret<String>>,
            #[serde(flatten)]
            user: LoginInfo,
        }

        let RegisterResponse { token, user } = self
            .request_with(Method::POST, "users", |b| b.json(&req))
            .await?;

        let new_self = token.map(|t| Self {
            client: self.client.clone(),
            instance_url: self.instance_url.clone(),
            lang: self.lang.clone(),
            token: Some(t),
        });

        Ok((new_self, user))
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

        let mut builder = self
            .client
            .request(method, &format!("{}/{}", self.instance_url, subpath));

        if let Some(lang) = &self.lang {
            builder = builder.header("Accept-Language", lang);
        };

        let mut builder = f(builder);

        if let Some(token) = &self.token {
            builder = builder.bearer_auth(token.expose_secret())
        };

        builder.send().await?.error_for_status()?.json().await
    }
}
