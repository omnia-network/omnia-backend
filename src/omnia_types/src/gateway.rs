use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::{cmp::Ordering, collections::BTreeMap};

use crate::{
    device::DeviceUid,
    environment::EnvironmentUID,
    errors::GenericResult,
    http::{Ip, ProxiedGatewayUID},
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

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct InitializedGatewayValue {
    pub principal_id: GatewayPrincipalId,
    pub proxied_gateway_uid: Option<String>,
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

pub type RegisteredGatewayResult = GenericResult<RegisteredGatewayValue>;
pub type MultipleRegisteredGatewayResult = GenericResult<Vec<RegisteredGatewayValue>>;
