use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct List<T> {
    pub items: Box<[T]>,
    pub next_page: Option<Box<str>>,
}
