use serde::Deserialize;

/// The `text`, `markdown`, and `html` content data found in various models
#[derive(Debug, Deserialize)]
pub struct Content {
    pub content_text: Option<Box<str>>,
    pub content_markdown: Option<Box<str>>,
    pub content_html: Option<Box<str>>,
}
