use std::{cmp::Ordering, collections::BTreeSet};

use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::{errors::GenericResult, gateway::{GatewayPrincipalId, GatewayURL}, environment::EnvironmentUID, affordance::AffordanceValue, http::ProxiedGatewayUID};

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
    pub environment: EnvironmentUID,
    pub affordances: BTreeSet<AffordanceValue>
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct DevicesAccessInfo {
    pub devices_uid: BTreeSet<DeviceUid>,
    pub proxied_gateway_uid: Option<ProxiedGatewayUID>,
    pub gateway_url: GatewayURL,
}

pub type RegisteredDeviceResult = GenericResult<RegisteredDeviceIndex>;

pub type RegisteredDeviceOption = Option<RegisteredDeviceValue>;
