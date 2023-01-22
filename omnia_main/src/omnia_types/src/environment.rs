use candid::{CandidType, Deserialize};

use crate::errors::GenericError;

pub type EnvironmentUID = String;

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
