use std::collections::BTreeMap;
use candid::{CandidType, Deserialize};

type PrincipalId = String;
type EnvironmentUID = String;
type GatewayUID = String;
type DeviceUID = String;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct DeviceInfo {
    pub device_name: String,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct GatewayInfo {
    pub gateway_name: String,
    pub gateway_uid: GatewayUID,
    pub devices: BTreeMap<DeviceUID, DeviceInfo>,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct EnvironmentInfo {
    pub env_name: String,
    pub env_uid: EnvironmentUID,
    pub env_gateways: BTreeMap<GatewayUID, GatewayInfo>,
    pub env_manager_principal_id: PrincipalId,
}