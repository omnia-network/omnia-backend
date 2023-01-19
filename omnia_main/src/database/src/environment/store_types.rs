use candid::{CandidType, Deserialize};
use omnia_types::{device::DeviceUID, gateway::GatewayUID, user::PrincipalId};
use std::collections::BTreeMap;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct StoredDeviceInfo {
    pub device_name: String,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct StoredGatewayInfo {
    pub gateway_name: String,
    pub devices: BTreeMap<DeviceUID, StoredDeviceInfo>,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct StoredEnvironmentInfo {
    pub env_name: String,
    pub env_gateways: BTreeMap<GatewayUID, StoredGatewayInfo>,
    pub env_manager_principal_id: PrincipalId,
}
