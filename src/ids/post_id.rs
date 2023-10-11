use reqwest::Method;

use crate::{
    api_models::{CommentInfo, JustId, List, MinimalAuthorInfo, PostInfo},
    prelude::{Client, ReqFlagPost, ReqPollVote, ReqPostReplies, ReqReply},
};

use super::{CommentId, PageId, PostId};

impl PostId {
    /// Request the full [`PostInfo`] object from the instance
    pub async fn to_post(&self, ctx: &Client) -> reqwest::Result<PostInfo> {
        ctx.request(Method::GET, &string_concat!("posts/", self.0.to_string()))
            .await
    }

    /// Delete the post
    pub async fn delete(&self, ctx: &Client) -> reqwest::Result<()> {
        ctx.request(
            Method::DELETE,
            &string_concat!("posts/", self.0.to_string()),
        )
        .await?;
        Ok(())
    }

    /// Like or unlike a post
    pub async fn vote(&self, ctx: &Client, is_like: bool) -> reqwest::Result<()> {
        ctx.request(
            match is_like {
                true => Method::PUT,
                false => Method::DELETE,
            },
            &string_concat!("posts/", self.0.to_string(), "/your_vote"),
        )
        .await?;
        Ok(())
    }

    /// List likers of a post
    pub async fn votes(
        &self,
        ctx: &Client,
        page: Option<PageId>,
    ) -> reqwest::Result<List<MinimalAuthorInfo>> {
        ctx.request_with(
            Method::GET,
            &string_concat!("posts/", self.0.to_string(), "/votes"),
            |b| {
                if let Some(page) = page {
                    b.query(&[("page", page)])
                } else {
                    b
                }
            },
        )
        .await
    }

    /// Flag a post for review
    pub async fn flag<'a>(&self, ctx: &Client, req: &ReqFlagPost<'a>) -> reqwest::Result<()> {
        ctx.request_with(
            Method::POST,
            &string_concat!("posts/", self.0.to_string(), "/flags"),
            |b| b.json(req),
        )
        .await?;
        Ok(())
    }

    /// Submit a vote on a poll
    pub async fn poll_vote(&self, ctx: &Client, req: &ReqPollVote) -> reqwest::Result<()> {
        ctx.request_with(
            Method::PUT,
            &string_concat!("posts/", self.0.to_string(), "/poll/your_vote"),
            |b| b.json(req),
        )
        .await?;
        Ok(())
    }

    /// List replies
    pub async fn replies(
        &self,
        ctx: &Client,
        req: &ReqPostReplies,
    ) -> reqwest::Result<List<CommentInfo>> {
        ctx.request_with(
            Method::PUT,
            &string_concat!("posts/", self.0.to_string(), "/poll/your_vote"),
            |b| b.json(req),
        )
        .await
    }

    /// Reply to a post
    pub async fn reply<'a>(
        &self,
        ctx: &Client,
        req: &ReqReply<'a>,
    ) -> reqwest::Result<JustId<CommentId>> {
        ctx.request_with(
            Method::POST,
            &string_concat!("posts/", self.0.to_string(), "/replies"),
            |b| b.json(req),
        )
        .await
    }
}
