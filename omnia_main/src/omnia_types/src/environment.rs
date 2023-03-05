use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::gateway::GatewayPrincipalId;
use crate::errors::GenericError;
use crate::user::VirtualPersonaPrincipalId;

pub type EnvironmentUID = String;
pub type Ip = String;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct Environment {
    pub env_name: String,
    pub env_ip: Option<Ip>,
    pub env_users_principals_ids: Vec<VirtualPersonaPrincipalId>,
    pub env_gateway_principal_ids: Vec<GatewayPrincipalId>,
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
    pub env_name: String,
    pub env_uid: EnvironmentUID,
    pub env_manager_principal_id: String,
}

pub type EnvironmentInfoResult = Result<EnvironmentInfo, GenericError>;
