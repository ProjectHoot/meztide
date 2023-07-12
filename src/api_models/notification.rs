use serde::Deserialize;

use super::{PostCommentInfo, PostListPost};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum NotificationInfo {
    PostReply {
        reply: PostCommentInfo,
        post: PostListPost,
    },
    CommentReply {
        reply: PostCommentInfo,
        comment: PostCommentInfo,
        post: PostListPost,
    },
}

#[derive(Debug, Deserialize)]
pub struct Notification {
    #[serde(flatten)]
    pub info: NotificationInfo,

    pub unseen: bool,
}
