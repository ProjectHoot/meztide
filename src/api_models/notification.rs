use serde::Deserialize;

use super::{CommentInfo, PostListPost};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum NotificationInfo {
    PostReply {
        reply: CommentInfo,
        post: PostListPost,
    },
    CommentReply {
        reply: CommentInfo,
        comment: CommentInfo,
        post: PostListPost,
    },
}

/// User notification data
#[derive(Debug, Deserialize)]
pub struct Notification {
    #[serde(flatten)]
    pub info: NotificationInfo,

    pub unseen: bool,
}
