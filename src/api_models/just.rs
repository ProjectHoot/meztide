use serde::Deserialize;

use super::MinimalAuthorInfo;

#[derive(Debug, Deserialize)]
pub struct JustUrl {
    pub url: Box<str>,
}

#[derive(Debug, Deserialize)]
pub struct JustUser {
    pub user: MinimalAuthorInfo,
}

#[derive(Debug, Deserialize)]
pub struct JustId<T> {
    pub id: T,
}

#[derive(Debug, Deserialize)]
pub struct JustContentText {
    pub content_text: Box<str>,
}
