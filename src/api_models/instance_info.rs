use serde::Deserialize;

use super::Content;

/// Info about the backend software running the instance
#[derive(Debug, Deserialize)]
pub struct InstanceSoftware {
    /// Example: "lotide"
    pub name: Box<str>,
    /// Example: "0.9.0-pre"
    pub version: Box<str>,
}

/// General info about the instance
#[derive(Debug, Deserialize)]
pub struct InstanceInfo {
    pub software: InstanceSoftware,
    pub description: Content,
    pub web_push_vapid_key: Box<str>,
    pub signup_allowed: bool,
    pub invitations_enabled: bool,
    pub community_creation_requirement: Option<Box<str>>,
    pub invitation_creation_requirement: Option<Box<str>>,
}
