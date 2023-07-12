use serde::Deserialize;

use crate::ids::FlagId;

use super::{JustContentText, MinimalAuthorInfo, PostListPost};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum FlagDetails {
    Post { post: PostListPost },
}

#[derive(Debug, Deserialize)]
pub struct FlagInfo {
    pub id: FlagId,

    pub flagger: MinimalAuthorInfo,

    pub created_local: Box<str>,

    pub content: Option<JustContentText>,

    #[serde(flatten)]
    pub details: FlagDetails,
}
