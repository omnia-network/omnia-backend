use std::cmp::Ordering;

use crate::errors::GenericResult;
use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::provisional::CanisterId;
use omnia_core_sdk::access_key::{AccessKeyUID, UniqueAccessKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccessKeyIndex {
    pub access_key_uid: AccessKeyUID,
}

impl Ord for AccessKeyIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.access_key_uid.cmp(&other.access_key_uid)
    }
}

impl PartialOrd for AccessKeyIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct AccessKeyValue {
    pub key: AccessKeyUID,
    pub owner: Principal,
    pub transaction_hash: TransactionHash,
    pub counter: u32,
    pub used_nonces: Vec<u128>,
}

impl Default for AccessKeyValue {
    fn default() -> Self {
        Self {
            key: "".to_string(),
            owner: Principal::anonymous(),
            transaction_hash: [0; 32],
            counter: 0,
            used_nonces: vec![],
        }
    }
}

impl AccessKeyValue {
    pub fn new(key: AccessKeyUID, owner: Principal, transaction_hash: TransactionHash) -> Self {
        Self {
            key,
            owner,
            transaction_hash,
            counter: 0,
            used_nonces: vec![],
        }
    }

    pub fn get_key(self) -> String {
        self.key
    }

    pub fn is_used_nonce(&self, nonce: u128) -> bool {
        self.used_nonces.contains(&nonce)
    }

    pub fn get_requests_count(&self) -> u32 {
        self.counter
    }

    pub fn spend_nonce(&mut self, nonce: u128) {
        self.used_nonces.push(nonce);
        self.counter += 1;
    }
}

/// Use [get_transaction_hash][omnia_utils::ic::get_transaction_hash] to generate the transaction hash
pub type TransactionHash = [u8; 32];

#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct AccessKeyCreationArgs {
    pub owner: Principal,
    pub transaction_hash: TransactionHash,
}
pub type AccessKeyCreationResult = GenericResult<AccessKeyValue>;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct SignedRequest {
    signature_hex: String,
    unique_access_key: UniqueAccessKey,
    requester_canister_id: CanisterId,
}

impl SignedRequest {
    pub fn get_signature(&self) -> String {
        self.signature_hex.clone()
    }

    pub fn get_unique_access_key(&self) -> UniqueAccessKey {
        self.unique_access_key.clone()
    }

    pub fn get_requester_principal_id(&self) -> CanisterId {
        self.requester_canister_id
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum RejectedAccessKeyReason {
    InvalidSignature,
    InvalidNonce,
    InvalidAccessKey,
    RequestsLimitReached,
    NonceAlreadyUsed,
    SignatureVerificationError(String),
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct RejectedAccessKey {
    pub key: AccessKeyUID,
    pub reason: RejectedAccessKeyReason,
}
