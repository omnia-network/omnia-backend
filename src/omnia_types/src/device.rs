use std::cmp::Ordering;

use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::{environment::EnvironmentUID, errors::GenericResult, gateway::GatewayPrincipalId};

pub type DeviceUid = String;
// TODO: change it to a URL type, so that we can validate it properly
pub type DeviceUrl = String;

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
    pub gateway_principal_id: GatewayPrincipalId,
    pub env_uid: EnvironmentUID,
    /// The publicly accessible URL of the device, use [get_device_url](omnia::utils::net::get_device_url) to generate it
    pub device_url: DeviceUrl,
    /// HTTP Headers to send along with the request to the device_url
    pub required_headers: Option<Vec<(String, String)>>,
}

pub type RegisteredDeviceResult = GenericResult<(RegisteredDeviceIndex, RegisteredDeviceValue)>;

pub type RegisteredDeviceOption = Option<RegisteredDeviceValue>;

pub type RegisteredDevicesUidsResult = GenericResult<RegisteredDevicesUids>;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
/// The device affordances read from the Thing Description
pub struct DeviceAffordances {
    pub properties: Vec<String>,
    pub actions: Vec<String>,
}
