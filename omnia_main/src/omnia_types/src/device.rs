use std::cmp::Ordering;

use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::{errors::GenericResult, gateway::GatewayPrincipalId, environment::EnvironmentUID};

pub type DeviceUid = String;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct RegisteredDeviceIndex {
    pub device_uid: DeviceUid,
}

impl Ord for RegisteredDeviceIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.device_uid.cmp(&other.device_uid)
    }
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct RegisteredDeviceValue {
    pub name: String,
    pub gateway_principal_id: GatewayPrincipalId,
    pub environment: EnvironmentUID
}

pub type RegisteredDeviceResult = GenericResult<RegisteredDeviceValue>;

pub type RegisteredDeviceOption = Option<RegisteredDeviceValue>;