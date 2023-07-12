use serde::Deserialize;

use crate::ids::PostId;

use super::{Empty, MinimalAuthorInfo, MinimalCommunityInfo, PollInfo};

#[derive(Debug, Deserialize)]
pub struct PostInfo {
    #[serde(flatten)]
    pub post: PostListPost,

    pub approved: bool,
    pub rejected: bool,
    pub local: bool,
    pub poll: Option<PollInfo>,
}

#[derive(Debug, Deserialize)]
pub struct PostListPost {
    pub id: PostId,
    pub title: Box<str>,
    pub remote_url: Option<Box<str>>,
    pub href: Option<Box<str>>,
    pub content_text: Option<Box<str>>,
    pub content_markdown: Option<Box<str>>,
    pub content_html: Option<String>,
    pub author: Option<MinimalAuthorInfo>,
    pub created: Box<str>,
    pub community: MinimalCommunityInfo,
    pub replies_count_total: Option<i64>,
    pub relevance: Option<f32>,
    pub score: i64,
    pub sticky: bool,
    pub your_vote: Option<Empty>,
    pub sensitive: bool,
}

#[derive(Debug, Deserialize)]
pub struct MinimalPostInfo {
    pub id: PostId,
    pub title: Box<str>,
    pub remote_url: Option<Box<str>>,
    pub sensitive: bool,
}
