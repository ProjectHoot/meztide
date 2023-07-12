use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AvatarInfo {
    pub url: Box<str>,
}
