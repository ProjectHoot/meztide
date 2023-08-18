//! Models used when making requests to the API

use serde::{Deserialize, Serialize};

use crate::prelude::{CommunityId, PageId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReqRegister<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email_address: Option<&'a str>,
    pub invitation_key: Option<&'a str>,
    pub login: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReqCommunitesSort {
    OldLocal,
    Alphabetic,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReqCommunities<'a> {
    /// If true, will include `you_are_moderator` and `your_follow` in list. Requires login.
    pub include_your: Option<bool>,
    pub search: Option<&'a str>,
    // pub your_follow: Accepted, // TODO
    /// Filter based on whether you are a moderator of the community. Requires login.
    pub you_are_moderator: Option<bool>,
    /// Filter to either local or remote communities.
    pub local: Option<bool>,
    pub limit: Option<u32>,
    pub page: Option<PageId>,
    pub sort: Option<ReqCommunitesSort>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReqPosts<'a> {
    pub include_your: Option<bool>,
    pub sort: Option<&'a str>,
    pub search: Option<&'a str>,
    /// Filter by whether the post is approved in a local community
    pub in_any_local_community: Option<bool>,
    /// If true, will omit posts from communities marked as hide_posts_from_aggregates
    pub use_aggregate_filters: Option<bool>,
    /// If present, will filter to posts approved in the specified community
    pub community: Option<CommunityId>,
    /// Filter by whether the post is approved in one of the communities you follow
    pub in_your_follows: Option<bool>,
    /// How far into the past to include posts from, as an ISO8601 duration.
    pub created_within: Option<&'a str>,
    /// If true, will sort sticky posts to the top
    pub sort_sticky: Option<bool>,
    pub limit: Option<u32>,
    pub page: Option<PageId>,
}
