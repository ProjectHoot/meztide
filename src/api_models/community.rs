use serde::Deserialize;

use crate::ids::CommunityId;

use super::Content;

#[derive(Debug, Deserialize)]
pub struct YourFollowInfo {
    /// Whether your follow request has been accepted by the community.
    pub accepted: bool,
}

#[derive(Debug, Deserialize)]
pub struct CommunityFeedsType {
    pub new: Box<str>,
}

#[derive(Debug, Deserialize)]
pub struct CommunityFeeds {
    pub atom: CommunityFeedsType,
}

/// Full community info
#[derive(Debug, Deserialize)]
pub struct CommunityInfo {
    #[serde(flatten)]
    pub base: MinimalCommunityInfo,

    pub description: Content,
    pub feeds: CommunityFeeds,

    pub you_are_moderator: Option<bool>,
    pub your_follow: Option<YourFollowInfo>,

    /// Number of pending flags sent to this community.
    /// Present with include_your=true if you are a moderator
    pub pending_moderation_actions: Option<u32>,
}

/// Community info contained in things like posts
#[derive(Debug, Deserialize)]
pub struct MinimalCommunityInfo {
    pub id: CommunityId,
    pub name: Box<str>,
    pub local: bool,
    pub host: Box<str>,
    pub remote_url: Option<Box<str>>,
    pub deleted: bool,
}
