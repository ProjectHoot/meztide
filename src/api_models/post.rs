use serde::Deserialize;

use crate::ids::PostId;

use super::{Content, Empty, MinimalAuthorInfo, MinimalCommunityInfo, PollInfo};

#[derive(Debug, Deserialize)]
pub struct PostInfo {
    #[serde(flatten)]
    pub post: PostListPost,

    pub approved: bool,
    pub rejected: bool,
    pub local: bool,
    pub poll: Option<PollInfo>,
}

impl PostInfo {
    #[inline]
    pub fn id(&self) -> PostId {
        self.post.id()
    }
}

#[derive(Debug, Deserialize)]
pub struct PostListPost {
    #[serde(flatten)]
    pub base: MinimalPostInfo,

    #[serde(flatten)]
    pub content: Content,

    pub href: Option<Box<str>>,
    pub author: Option<MinimalAuthorInfo>,
    pub created: Box<str>,
    pub community: MinimalCommunityInfo,
    pub replies_count_total: Option<i64>,
    pub relevance: Option<f32>,
    pub score: i64,
    pub sticky: bool,
    pub your_vote: Option<Empty>,
}

impl PostListPost {
    #[inline]
    pub fn id(&self) -> PostId {
        self.base.id
    }
}

#[derive(Debug, Deserialize)]
pub struct MinimalPostInfo {
    pub id: PostId,
    pub title: Box<str>,
    pub remote_url: Option<Box<str>>,
    pub sensitive: bool,
}
