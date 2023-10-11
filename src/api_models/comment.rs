use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use crate::ids::CommentId;

use super::{Empty, JustId, JustUrl, List, MinimalAuthorInfo, MinimalPostInfo};

#[derive(Debug, Deserialize)]
pub struct MinimalCommentInfo {
    pub id: CommentId,
    pub remote_url: Option<Box<str>>,
    pub sensitive: bool,
    pub content_text: Option<Box<str>>,
    pub content_html: Option<Box<str>>,
}

#[derive(Debug, Deserialize)]
pub struct CommentInfo {
    #[serde(flatten)]
    pub base: MinimalCommentInfo,

    pub attachments: Box<[JustUrl]>,
    pub author: Option<MinimalAuthorInfo>,
    pub content_markdown: Option<Box<str>>,
    pub created: DateTime<FixedOffset>,
    pub deleted: bool,
    pub local: bool,
    pub replies: Option<List<CommentInfo>>,
    pub score: i64,
    pub your_vote: Option<Empty>,
    pub parent: Option<JustId<CommentId>>,
    pub post: Option<MinimalPostInfo>,
}

#[derive(Debug, Deserialize)]
pub struct CommentFromUser {
    pub r#type: Box<str>,

    #[serde(flatten)]
    pub base: MinimalCommentInfo,

    pub created: DateTime<FixedOffset>,
    pub post: MinimalPostInfo,
}
