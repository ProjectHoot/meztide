use reqwest::Method;

use crate::{
    api_models::{CommunityInfo, ModeratorInfo},
    ctx::Ctx,
};

use super::{CommunityId, UserId};

impl CommunityId {
    /// Request the full [`Community`] object from the instance
    pub async fn to_community(&self, ctx: &Ctx) -> reqwest::Result<CommunityInfo> {
        ctx.request(Method::GET, &format!("communities/{}", self.0))
            .await
    }

    /// Delete the community
    pub async fn delete(&self, ctx: &Ctx) -> reqwest::Result<()> {
        ctx.request(Method::DELETE, &format!("communities/{}", self.0))
            .await?;
        Ok(())
    }

    /// Follow the community
    pub async fn follow(&self, ctx: &Ctx, try_wait_for_accept: bool) -> reqwest::Result<()> {
        #[derive(serde::Serialize)]
        struct FollowRequest {
            try_wait_for_accept: bool,
        }

        ctx.request_with(Method::POST, &format!("communities/{}", self.0), |b| {
            b.json(&FollowRequest {
                try_wait_for_accept,
            })
        })
        .await?;
        Ok(())
    }

    /// Unfollow the community
    pub async fn unfollow(&self, ctx: &Ctx) -> reqwest::Result<()> {
        ctx.request(Method::POST, &format!("communities/{}/unfollow", self.0))
            .await?;
        Ok(())
    }

    /// List moderators of the community
    pub async fn get_moderators(&self, ctx: &Ctx) -> reqwest::Result<Box<[ModeratorInfo]>> {
        ctx.request(Method::GET, &format!("communities/{}/moderators", self.0))
            .await
    }

    /// Add a moderator to the community
    pub async fn add_moderator(&self, ctx: &Ctx, user_id: UserId) -> reqwest::Result<()> {
        ctx.request(
            Method::PUT,
            &format!("communities/{}/moderators/{}", self.0, user_id.0),
        )
        .await?;
        Ok(())
    }
}
