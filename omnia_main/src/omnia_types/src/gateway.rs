use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::{cmp::Ordering, collections::BTreeMap};

use crate::{environment::EnvironmentUID, http::Ip, errors::GenericError, device::DeviceUid};

pub type GatewayUID = String;
pub type GatewayPrincipald = String;
pub type GatewayPrincipalId = String;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct InitializedGatewayIndex {
    pub ip: Ip,
}

impl Ord for InitializedGatewayIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ip.cmp(&other.ip)
    }
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct InitializedGatewayValue {
    pub principal_id: GatewayPrincipalId,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GatewayRegistrationInput {
    pub env_uid: EnvironmentUID,
    pub gateway_name: String,
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct RegisteredGatewayIndex {
    pub principal_id: GatewayPrincipalId,
}

impl Ord for RegisteredGatewayIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.principal_id.cmp(&other.principal_id)
    }
}


#[derive(Debug, Clone, CandidType, Default, Deserialize, Serialize)]
pub struct RegisteredGatewayValue {
    pub gateway_name: String,
    pub gateway_ip: Ip,
    pub env_uid: EnvironmentUID,
    pub gat_registered_device_uids: BTreeMap<DeviceUid, ()>,    // TODO: DeviceInfo
}

pub type RegisteredGatewayResult = Result<RegisteredGatewayValue, GenericError>;
pub type MultipleRegisteredGatewayResult = Result<Vec<RegisteredGatewayValue>, GenericError>;
