//! A Rust SDK for [lotide](https://github.com/lotide-org/lotide)
//!
//! ## Usage
//!
//! ```rust,no_run
//! use meztide::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("https://c.tide.tk/api/unstable");
//!
//!     let communities = client.communities(&Default::default()).await.unwrap();
//!
//!     let posts = client.posts(&Default::default()).await.unwrap();
//!
//!     let community_id = CommunityId(1);
//!
//!     // Get community data
//!     let community = community_id.to_community(&client).await.unwrap();
//!
//!     // Get community moderators
//!     let moderators = community_id.moderators(&client).await.unwrap();
//!
//!     // Follow the community
//!     let _ = community_id.follow(&client, false).await.unwrap();
//!
//!     // Delete the community
//!     let _ = community_id.delete(&client).await.unwrap();
//! }
//! ```
//!
//! See [`Client`](client::Client) or
//! [`ids`] for more things you can do.
//!
//! See [`api_models`] for more sights you can see.

#[macro_use]
extern crate string_concat;

pub mod api_models;
pub mod client;
pub mod ids;
pub mod request_models;

/// Convenient re-exports
pub mod prelude {
    pub use crate::{client::Client, ids::*, request_models::*};
}
