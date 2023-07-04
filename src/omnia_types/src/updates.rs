use std::{borrow::Cow, cmp::Ordering};

use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Serialize;

use crate::{
    errors::GenericResult,
    gateway::GatewayPrincipalId,
    virtual_persona::{VirtualPersonaIp, VirtualPersonaPrincipalId},
    MAX_STABLE_BTREE_MAP_SIZE,
};

pub type PairingPayload = String;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateIndex {
    pub gateway_principal_id: GatewayPrincipalId,
}

impl Ord for UpdateIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.gateway_principal_id.cmp(&other.gateway_principal_id)
    }
}

impl PartialOrd for UpdateIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Storable for UpdateIndex {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for UpdateIndex {
    const MAX_SIZE: u32 = MAX_STABLE_BTREE_MAP_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct UpdateValue {
    pub virtual_persona_principal_id: VirtualPersonaPrincipalId,
    pub virtual_persona_ip: VirtualPersonaIp,
    pub command: String,
    pub info: PairingInfo,
}

impl Storable for UpdateValue {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for UpdateValue {
    const MAX_SIZE: u32 = MAX_STABLE_BTREE_MAP_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct PairingInfo {
    pub payload: PairingPayload,
}

pub type UpdateValueResult = GenericResult<UpdateValue>;

pub type UpdateValueOption = Option<UpdateValue>;
