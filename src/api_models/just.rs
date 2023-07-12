use serde::Deserialize;

use super::MinimalAuthorInfo;

/// Just a `url: Box<str>` field
#[derive(Debug, Deserialize)]
pub struct JustUrl {
    pub url: Box<str>,
}

/// Just a `user: MinimalAuthorInfo` field
#[derive(Debug, Deserialize)]
pub struct JustUser {
    pub user: MinimalAuthorInfo,
}

/// Just a `id: T` field
#[derive(Debug, Deserialize)]
pub struct JustId<T> {
    pub id: T,
}

/// Just a `content_text: Box<str>` field
#[derive(Debug, Deserialize)]
pub struct JustContentText {
    pub content_text: Box<str>,
}
