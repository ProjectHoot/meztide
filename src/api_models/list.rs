use serde::Deserialize;

use crate::ids::PageId;

/// Used for pagination
///
/// `next_page` can be passed into any endpoint that returns a [`List`],
/// often a url query parameter called `page`.
#[derive(Debug, Deserialize)]
pub struct List<T> {
    pub items: Box<[T]>,
    pub next_page: Option<PageId>,
}
