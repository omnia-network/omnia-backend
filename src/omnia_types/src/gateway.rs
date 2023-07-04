use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Serialize;
use std::{borrow::Cow, cmp::Ordering, collections::BTreeMap};

use crate::{
    device::DeviceUid,
    environment::EnvironmentUID,
    errors::GenericResult,
    http::{Ip, ProxiedGatewayUID},
    MAX_STABLE_BTREE_MAP_SIZE,
};

pub type GatewayUID = String;
pub type GatewayPrincipald = String;
pub type GatewayPrincipalId = String;
// TODO: change it to a URL type, so that we can validate it properly
pub type GatewayUrl = String;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub struct InitializedGatewayIndex {
    pub ip: Ip,
}

impl Ord for InitializedGatewayIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ip.cmp(&other.ip)
    }
}

impl PartialOrd for InitializedGatewayIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Storable for InitializedGatewayIndex {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for InitializedGatewayIndex {
    const MAX_SIZE: u32 = MAX_STABLE_BTREE_MAP_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct InitializedGatewayValue {
    pub principal_id: GatewayPrincipalId,
    pub proxied_gateway_uid: Option<String>,
}

impl Storable for InitializedGatewayValue {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for InitializedGatewayValue {
    const MAX_SIZE: u32 = MAX_STABLE_BTREE_MAP_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GatewayRegistrationInput {
    pub env_uid: EnvironmentUID,
    pub gateway_name: String,
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegisteredGatewayIndex {
    pub principal_id: GatewayPrincipalId,
}

impl Ord for RegisteredGatewayIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.principal_id.cmp(&other.principal_id)
    }
}

impl PartialOrd for RegisteredGatewayIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Storable for RegisteredGatewayIndex {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for RegisteredGatewayIndex {
    const MAX_SIZE: u32 = MAX_STABLE_BTREE_MAP_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(Debug, Clone, CandidType, Default, Deserialize, Serialize)]
pub struct RegisteredGatewayValue {
    pub gateway_name: String,
    /// public IP of the gateway
    pub gateway_ip: Ip,
    /// URL of the proxy
    /// TODO: avoid storing it, becuase it can be derived from the gateway_ip
    pub gateway_url: GatewayUrl,
    pub proxied_gateway_uid: Option<ProxiedGatewayUID>,
    pub env_uid: EnvironmentUID,
    pub gat_registered_device_uids: BTreeMap<DeviceUid, ()>, // TODO: DeviceInfo
                                                             // TODO: add a is_proxied field to avoid having to check if proxied_gateway_uid is None and improve readability
                                                             // not sure if this is a good idea, because it would be another field stored in the DB
}

impl Storable for RegisteredGatewayValue {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for RegisteredGatewayValue {
    const MAX_SIZE: u32 = MAX_STABLE_BTREE_MAP_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

pub type RegisteredGatewayResult = GenericResult<RegisteredGatewayValue>;
pub type MultipleRegisteredGatewayResult = GenericResult<Vec<RegisteredGatewayValue>>;
