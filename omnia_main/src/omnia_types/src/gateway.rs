use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::cmp::Ordering;

use crate::{environment::EnvironmentUID, http::Ip, errors::GenericError};

pub type GatewayUID = String;
pub type GatewayPrincipald = String;
pub type GatewayPrincipalId = String;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct InitializedGatewayIndex {
    pub ip: Ip,
}

impl Ord for InitializedGatewayIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ip.cmp(&other.ip)
    }
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct InitializedGatewayValue {
    pub principal_id: GatewayPrincipalId,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GatewayRegistrationInput {
    pub env_uid: EnvironmentUID,
    pub gateway_name: String,
}

#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct RegisteredGateway {
    pub gateway_name: String,
    pub gateway_ip: Ip,
    pub env_uid: EnvironmentUID,

}

pub type RegisteredGatewayResult = Result<RegisteredGateway, GenericError>;
pub type MultipleRegisteredGatewayResult = Result<Vec<RegisteredGateway>, GenericError>;
