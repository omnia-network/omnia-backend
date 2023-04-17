use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::BTreeMap;

use crate::errors::GenericError;
use crate::gateway::GatewayPrincipalId;
use crate::http::Ip;
use crate::virtual_persona::VirtualPersonaPrincipalId;

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

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct EnvironmentValue {
    pub env_name: String,
    pub env_ip: Option<Ip>,
    pub env_users_principals_ids: BTreeMap<VirtualPersonaPrincipalId, ()>, // TODO: VirtualPersonaInfo
    pub env_gateways_principals_ids: BTreeMap<GatewayPrincipalId, ()>,     // TODO: GatewayInfo
    pub env_manager_principal_id: VirtualPersonaPrincipalId,
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

pub type EnvironmentInfoResult = Result<EnvironmentInfo, GenericError>;

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

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct EnvironmentUidValue {
    pub env_uid: EnvironmentUID,
}
