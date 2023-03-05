use std::collections::BTreeMap;
use candid::{CandidType, Deserialize};
use serde::Serialize;


use crate::{environment::EnvironmentUID, errors::GenericError};
use crate::device::{DeviceUID, StoredDeviceInfo};

pub type GatewayUID = String;
pub type GatewayPrincipald = String;
pub type GatewayIp = String;
pub type GatewayPrincipalId = String;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct StoredRegisteredgateway {
    pub gateway_name: String,
    pub devices: BTreeMap<DeviceUID, StoredDeviceInfo>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GatewayRegistrationInput {
    pub env_uid: EnvironmentUID,
    pub gateway_name: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct Registeredgateway {
    pub gateway_name: String,
    pub gateway_ip: GatewayIp,
    pub env_uid: EnvironmentUID,

}

pub type RegisteredgatewayResult = Result<Option<Registeredgateway>, GenericError>;
pub type MultipleRegisteredgatewayResult = Result<Vec<Registeredgateway>, GenericError>;
