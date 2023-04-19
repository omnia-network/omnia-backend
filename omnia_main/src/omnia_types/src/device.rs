use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};

use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::{
    affordance::AffordanceValue, environment::EnvironmentUID, errors::GenericResult,
    gateway::GatewayPrincipalId,
};

pub type DeviceUid = String;

pub type RegisteredDevicesUids = Vec<DeviceUid>;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegisteredDeviceIndex {
    pub device_uid: DeviceUid,
}

impl Ord for RegisteredDeviceIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.device_uid.cmp(&other.device_uid)
    }
}

impl PartialOrd for RegisteredDeviceIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct RegisteredDeviceValue {
    pub name: String,
    pub gateway_principal_id: GatewayPrincipalId,
    pub environment: EnvironmentUID,
    pub affordances: BTreeSet<AffordanceValue>,
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct DevicesAccessInfo {
    pub devices_urls: Vec<String>,
    pub required_headers: BTreeMap<String, String>,
}

pub type RegisteredDeviceResult = GenericResult<RegisteredDeviceIndex>;

pub type RegisteredDeviceOption = Option<RegisteredDeviceValue>;

pub type RegisteredDevicesUidsResult = GenericResult<RegisteredDevicesUids>;