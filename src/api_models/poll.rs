use serde::Deserialize;

use crate::ids::PollOptionId;

use super::JustId;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PollVoteBody {
    Multiple { options: Box<[PollOptionId]> },
    Single { option: PollOptionId },
}

#[derive(Debug, Deserialize)]
pub struct PollYourVote {
    pub options: Box<[JustId<PollOptionId>]>,
}

#[derive(Debug, Deserialize)]
pub struct PollOption {
    pub id: PollOptionId,
    pub name: Box<str>,
    pub votes: u32,
}

#[derive(Debug, Deserialize)]
pub struct PollInfo {
    pub multiple: bool,
    pub options: Box<[PollOption]>,
    pub your_vote: Option<PollYourVote>,
    pub closed_at: Option<Box<str>>,
    pub is_closed: bool,
}
