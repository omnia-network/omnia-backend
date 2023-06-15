use std::cmp::Ordering;

use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::errors::GenericResult;

pub type RequestKeyUID = String;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub struct RequestKeyIndex {
    pub request_key_uid: RequestKeyUID,
}

impl Ord for RequestKeyIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.request_key_uid.cmp(&other.request_key_uid)
    }
}

impl PartialOrd for RequestKeyIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default, Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct RequestKeyValue {
    pub key: RequestKeyUID,
    pub owner: String,
    pub counter: u128,
}

impl RequestKeyValue {
    pub fn new(key: RequestKeyUID, owner: String) -> Self {
        Self {
            key,
            owner,
            counter: 0,
        }
    }
}

pub type RequestKeyCreationResult = GenericResult<RequestKeyValue>;
