use candid::{CandidType, Deserialize};

type EnvironmentUID = String;
type GatewayUID = String;
type DeviceUID = String;

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
pub struct GatewayRegistrationInput {
    pub env_uid: EnvironmentUID,
    pub gateway_name: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GatewayRegistrationResult {
    pub gateway_name: String,
    pub gateway_uid: GatewayUID,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct DeviceRegistrationInput {
    pub env_uid: EnvironmentUID,
    pub gateway_uid: GatewayUID,
    pub device_name: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct DeviceRegistrationResult {
    pub device_name: String,
    pub device_uid: DeviceUID,
}