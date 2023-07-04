use std::{borrow::Cow, cmp::Ordering};

use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Serialize;

use crate::{
    environment::EnvironmentUID, errors::GenericResult, gateway::GatewayPrincipalId,
    MAX_STABLE_BTREE_MAP_SIZE,
};

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

impl Storable for RegisteredDeviceIndex {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for RegisteredDeviceIndex {
    const MAX_SIZE: u32 = MAX_STABLE_BTREE_MAP_SIZE;
    const IS_FIXED_SIZE: bool = false;
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

impl Storable for RegisteredDeviceValue {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for RegisteredDeviceValue {
    const MAX_SIZE: u32 = MAX_STABLE_BTREE_MAP_SIZE;
    const IS_FIXED_SIZE: bool = false;
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
