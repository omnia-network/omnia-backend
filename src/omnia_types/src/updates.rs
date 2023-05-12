use std::cmp::Ordering;

use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::{
    errors::GenericResult,
    gateway::GatewayPrincipalId,
    virtual_persona::{VirtualPersonaIp, VirtualPersonaPrincipalId},
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

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct UpdateValue {
    pub virtual_persona_principal_id: VirtualPersonaPrincipalId,
    pub virtual_persona_ip: VirtualPersonaIp,
    pub command: String,
    pub info: PairingInfo,
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct PairingInfo {
    pub payload: PairingPayload,
}

pub type UpdateValueResult = GenericResult<UpdateValue>;

pub type UpdateValueOption = Option<UpdateValue>;
