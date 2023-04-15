use std::cmp::Ordering;

use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::{
    gateway::GatewayPrincipalId,
    errors::GenericResult,
    virtual_persona::{VirtualPersonaIp, VirtualPersonaPrincipalId}
};

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct UpdateIndex {
    pub gateway_principal_id: GatewayPrincipalId,
}

impl Ord for UpdateIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.gateway_principal_id.cmp(&other.gateway_principal_id)
    }
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct UpdateValue {
    pub virtual_persona_principal_id: VirtualPersonaPrincipalId,
    pub virtual_persona_ip: VirtualPersonaIp,
    pub command: String,
    pub info: PairingInfo
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct PairingInfo {
    pub payload: String 
}

pub type UpdateValueResult = GenericResult<UpdateValue>;

pub type UpdateValueOption = Option<UpdateValue>;