use serde::Deserialize;

use super::Content;

#[derive(Debug, Deserialize)]
pub struct InstanceSoftware {
    /// Example: "lotide"
    pub name: Box<str>,
    /// Example: "0.9.0-pre"
    pub version: Box<str>,
}

#[derive(Debug, Deserialize)]
pub struct InstanceInfo {
    pub software: InstanceSoftware,
    pub description: Content,
    pub web_push_vapid_key: Box<str>,
    pub signup_allowed: bool,
}
