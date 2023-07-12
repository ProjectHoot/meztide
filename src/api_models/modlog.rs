use serde::Deserialize;

use super::{MinimalAuthorInfo, MinimalCommunityInfo, MinimalPostInfo};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum SiteModlogEventDetails {
    DeletePost {
        author: MinimalAuthorInfo,
        community: MinimalCommunityInfo,
    },
    DeleteComment {
        author: MinimalAuthorInfo,
        post: MinimalPostInfo,
    },
    SuspendUser {
        user: MinimalAuthorInfo,
    },
    UnsuspendUser {
        user: MinimalAuthorInfo,
    },
}

#[derive(Debug, Deserialize)]
pub struct SiteModlogEvent {
    pub time: Box<str>,

    #[serde(flatten)]
    pub details: SiteModlogEventDetails,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum CommunityModlogEventDetails {
    RejectPost { post: MinimalPostInfo },
    ApprovePost { post: MinimalPostInfo },
}

#[derive(Debug, Deserialize)]
pub struct CommunityModlogEvent {
    pub time: Box<str>,

    #[serde(flatten)]
    pub details: CommunityModlogEventDetails,
}
