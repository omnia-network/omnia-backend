use std::cmp::Ordering;

use candid::CandidType;
use ic_cdk::api::management_canister::provisional::CanisterId;
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
    key: RequestKeyUID,
    owner: String,
    counter: u128,
    used_nonces: Vec<u128>,
}

impl RequestKeyValue {
    pub fn new(key: RequestKeyUID, owner: String) -> Self {
        Self {
            key,
            owner,
            counter: 0,
            used_nonces: vec![],
        }
    }

    pub fn increment_counter(self) -> Self {
        Self {
            counter: self.counter + 1,
            ..self
        }
    }

    pub fn get_key(self) -> String {
        self.key
    }

    pub fn is_used_nonce(&self, nonce: u128) -> bool {
        self.used_nonces.contains(&nonce)
    }
}

pub type RequestKeyCreationResult = GenericResult<RequestKeyValue>;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct SignedRequest {
    signature_hex: String,
    unique_request_key: UniqueRequestKey,
    requester_canister_id: CanisterId,
}

impl SignedRequest {
    pub fn get_signature(&self) -> String {
        self.signature_hex.clone()
    }

    pub fn get_unique_request_key(&self) -> UniqueRequestKey {
        self.unique_request_key.clone()
    }

    pub fn get_requester_principal_id(&self) -> CanisterId {
        self.requester_canister_id
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct UniqueRequestKey {
    nonce: u128,
    uid: RequestKeyUID,
}

impl UniqueRequestKey {
    pub fn get_nonce(&self) -> u128 {
        self.nonce
    }

    pub fn get_uid(&self) -> RequestKeyUID {
        self.uid.clone()
    }
}
