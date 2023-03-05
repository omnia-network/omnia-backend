use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::environment::EnvironmentUID;
use crate::errors::GenericError;
use crate::gateway::GatewayUID;

pub type DeviceUID = String;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct StoredDeviceInfo {
    pub device_name: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct DeviceRegistrationInput {
    pub env_uid: EnvironmentUID,
    pub gateway_uid: GatewayUID,
    pub device_name: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct DeviceInfo {
    pub device_name: String,
    pub device_uid: DeviceUID,
    pub gateway_uid: GatewayUID,
}

pub type DeviceInfoResult = Result<DeviceInfo, GenericError>;
pub type MultipleDeviceInfoResult = Result<Vec<DeviceInfo>, GenericError>;
