use serde::Deserialize;

/// URL to a user avatar
#[derive(Debug, Deserialize)]
pub struct AvatarInfo {
    pub url: Box<str>,
}
