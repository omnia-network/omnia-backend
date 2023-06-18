use std::cmp::Ordering;

use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::provisional::CanisterId;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sha2::Sha256;

use crate::errors::GenericResult;

pub type AccessKeyUID = String;

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
    pub counter: u128,
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
pub struct UniqueAccessKey {
    nonce: u128,
    key: AccessKeyUID,
}

impl UniqueAccessKey {
    pub fn new(nonce: u128, key: AccessKeyUID) -> Self {
        Self { nonce, key }
    }

    pub fn get_nonce(&self) -> u128 {
        self.nonce
    }

    pub fn get_key(&self) -> AccessKeyUID {
        self.key.clone()
    }

    /// Serialize the UniqueAccessKey to a string
    pub fn serialize(&self) -> String {
        to_string(self).expect("UniqueAccessKey serialization failed")
    }

    /// Generate a **sha256** hash of the UniqueAccessKey
    pub fn generate_hash(&self) -> [u8; 32] {
        use sha2::Digest;
        let mut hasher = Sha256::new();
        hasher.update(self.serialize().as_bytes());
        hasher.finalize().into()
    }
}
