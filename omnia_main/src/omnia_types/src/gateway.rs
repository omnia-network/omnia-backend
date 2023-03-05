use std::collections::BTreeMap;
use candid::{CandidType, Deserialize};
use serde::Serialize;


use crate::{environment::EnvironmentUID, errors::GenericError};
use crate::device::{DeviceUID, StoredDeviceInfo};

pub type GatewayUID = String;
pub type GatewayIp = String;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct StoredGatewayInfo {
    pub gateway_name: String,
    pub devices: BTreeMap<DeviceUID, StoredDeviceInfo>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GatewayRegistrationInput {
    pub env_uid: EnvironmentUID,
    pub gateway_name: String,
    pub gateway_uid: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GatewayInfo {
    pub gateway_name: String,
    pub gateway_uid: GatewayUID,
}

pub type GatewayInfoResult = Result<Option<GatewayInfo>, GenericError>;
pub type MultipleGatewayInfoResult = Result<Vec<GatewayInfo>, GenericError>;
