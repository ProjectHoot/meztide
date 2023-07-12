use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Content {
    pub content_text: Box<str>,
    pub content_markdown: Box<str>,
    pub content_html: Box<str>,
}
