use std::{borrow::Cow, cmp::Ordering};

use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Serialize;

use crate::{environment::EnvironmentUID, errors::GenericResult, MAX_STABLE_BTREE_MAP_SIZE};

pub type VirtualPersonaPrincipalId = String;

pub type VirtualPersonaIp = String;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub struct VirtualPersonaIndex {
    pub principal_id: VirtualPersonaPrincipalId,
}

impl Ord for VirtualPersonaIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.principal_id.cmp(&other.principal_id)
    }
}

impl PartialOrd for VirtualPersonaIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Storable for VirtualPersonaIndex {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for VirtualPersonaIndex {
    const MAX_SIZE: u32 = MAX_STABLE_BTREE_MAP_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct VirtualPersonaValue {
    pub virtual_persona_principal_id: VirtualPersonaPrincipalId,
    pub virtual_persona_ip: VirtualPersonaIp,
    pub user_env_uid: Option<EnvironmentUID>,
    pub manager_env_uid: Option<EnvironmentUID>,
}

impl Storable for VirtualPersonaValue {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for VirtualPersonaValue {
    const MAX_SIZE: u32 = MAX_STABLE_BTREE_MAP_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

pub type VirtualPersonaValueResult = GenericResult<VirtualPersonaValue>;
