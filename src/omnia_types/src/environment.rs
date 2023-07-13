use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Serialize;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::BTreeMap;

use crate::errors::GenericResult;
use crate::gateway::GatewayPrincipalId;
use crate::http::Ip;
use crate::virtual_persona::VirtualPersonaPrincipalId;
use crate::MAX_STABLE_BTREE_MAP_SIZE;

pub type EnvironmentUID = String;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub struct EnvironmentIndex {
    pub environment_uid: EnvironmentUID,
}

impl Ord for EnvironmentIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.environment_uid.cmp(&other.environment_uid)
    }
}

impl PartialOrd for EnvironmentIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Storable for EnvironmentIndex {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for EnvironmentIndex {
    const MAX_SIZE: u32 = 1000;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct EnvironmentValue {
    pub env_name: String,
    pub env_ip: Option<Ip>,
    pub env_users_principals_ids: BTreeMap<VirtualPersonaPrincipalId, ()>, // TODO: VirtualPersonaInfo
    pub env_gateways_principals_ids: BTreeMap<GatewayPrincipalId, ()>,     // TODO: GatewayInfo
    pub env_manager_principal_id: VirtualPersonaPrincipalId,
}

impl Storable for EnvironmentValue {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for EnvironmentValue {
    const MAX_SIZE: u32 = 1000;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(Debug, CandidType, Deserialize)]
pub struct EnvironmentCreationInput {
    pub env_name: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct EnvironmentCreationResult {
    pub env_name: String,
    pub env_uid: EnvironmentUID,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct EnvironmentInfo {
    pub env_uid: EnvironmentUID,
}

pub type EnvironmentInfoResult = GenericResult<EnvironmentInfo>;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub struct EnvironmentUidIndex {
    pub ip: Ip,
}

impl Ord for EnvironmentUidIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ip.cmp(&other.ip)
    }
}

impl PartialOrd for EnvironmentUidIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Storable for EnvironmentUidIndex {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for EnvironmentUidIndex {
    const MAX_SIZE: u32 = MAX_STABLE_BTREE_MAP_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct EnvironmentUidValue {
    pub env_uid: EnvironmentUID,
}

impl Storable for EnvironmentUidValue {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for EnvironmentUidValue {
    const MAX_SIZE: u32 = MAX_STABLE_BTREE_MAP_SIZE;
    const IS_FIXED_SIZE: bool = false;
}
