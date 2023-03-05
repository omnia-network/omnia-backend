use std::collections::BTreeMap;
use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::gateway::{GatewayUID, StoredGatewayInfo};
use crate::errors::GenericError;
use crate::user::PrincipalId;

pub type EnvironmentUID = String;
pub type Ip = String;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct Environment {
    pub env_name: String,
    pub env_ip: Option<Ip>,
    pub env_gateways: BTreeMap<GatewayUID, StoredGatewayInfo>,
    pub env_manager_principal_id: PrincipalId,
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
    pub env_name: String,
    pub env_uid: EnvironmentUID,
    pub env_manager_principal_id: String,
}

pub type EnvironmentInfoResult = Result<EnvironmentInfo, GenericError>;
