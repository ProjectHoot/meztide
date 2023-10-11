use serde::Deserialize;

use crate::ids::UserId;

use super::{AvatarInfo, CommentFromUser, Content, PostListPost};

/// Basic data about a user
///
/// Note: perhaps this should be called `MinimalUserInfo`?
#[derive(Debug, Deserialize)]
pub struct MinimalAuthorInfo {
    pub id: UserId,
    pub username: Box<str>,
    pub local: bool,
    pub host: Box<str>,
    pub remote_url: Option<Box<str>>,
    pub is_bot: bool,
    pub avatar: Option<AvatarInfo>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    #[serde(flatten)]
    pub base: MinimalAuthorInfo,

    pub description: Content,
    /// Private note about this user written by you
    pub your_note: Option<Box<str>>,
    /// Whether the user is suspended
    pub suspended: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ModeratorInfo {
    #[serde(flatten)]
    pub base: MinimalAuthorInfo,
    pub moderator_since: Option<Box<str>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum UserThing {
    Comment(CommentFromUser),
    Post(PostListPost),
}
