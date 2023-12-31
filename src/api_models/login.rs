use serde::{Deserialize, Serialize};

use crate::ids::UserId;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginUserInfo {
    pub id: UserId,
    pub username: Box<str>,
    pub is_site_admin: bool,
    pub has_unread_notifications: bool,
    pub has_pending_moderation_actions: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginInfo {
    pub user: LoginUserInfo,
    pub permissions: Option<LoginPermissions>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginPermissions {
    pub create_community: PermissionInfo,
    pub create_invitation: PermissionInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PermissionInfo {
    pub allowed: bool,
}
