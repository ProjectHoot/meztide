use serde::Deserialize;

use crate::ids::CommentId;

use super::{Empty, JustId, JustUrl, List, MinimalAuthorInfo, MinimalPostInfo};

#[derive(Debug, Deserialize)]
pub struct MinimalCommentInfo {
    pub id: CommentId,
    pub remote_url: Option<Box<str>>,
    pub sensitive: bool,
    pub content_text: Option<Box<str>>,
    pub content_html: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PostCommentInfo {
    #[serde(flatten)]
    pub base: MinimalCommentInfo,

    pub attachments: Vec<JustUrl>,
    pub author: Option<MinimalAuthorInfo>,
    pub content_markdown: Option<Box<str>>,
    pub created: String,
    pub deleted: bool,
    pub local: bool,
    pub replies: Option<List<PostCommentInfo>>,
    pub score: i64,
    pub your_vote: Option<Empty>,
}

#[derive(Debug, Deserialize)]
pub struct CommentInfo {
    #[serde(flatten)]
    pub base: PostCommentInfo,

    pub parent: Option<JustId<CommentId>>,
    pub post: Option<MinimalPostInfo>,
}
