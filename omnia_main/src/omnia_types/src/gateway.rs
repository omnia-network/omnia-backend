use candid::{CandidType, Deserialize};

use crate::environment::EnvironmentUID;

pub type GatewayUID = String;

#[derive(Debug, CandidType, Deserialize)]
pub struct GatewayRegistrationInput {
    pub env_uid: EnvironmentUID,
    pub gateway_name: String,
    pub gateway_uid: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GatewayRegistrationResult {
    pub gateway_name: String,
    pub gateway_uid: GatewayUID,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GatewayInfo {
    pub gateway_name: String,
    pub gateway_uid: GatewayUID,
}
