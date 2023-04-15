use std::cmp::Ordering;

use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::{environment::EnvironmentUID, errors::GenericError};

pub type VirtualPersonaPrincipalId = String;

pub type VirtualPersonaIp = String;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct VirtualPersonaIndex {
    pub principal_id: VirtualPersonaPrincipalId,
}

impl Ord for VirtualPersonaIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.principal_id.cmp(&other.principal_id)
    }
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct VirtualPersonaValue {
    pub virtual_persona_principal_id: VirtualPersonaPrincipalId,
    pub virtual_persona_ip: VirtualPersonaIp,
    pub user_env_uid: Option<EnvironmentUID>,
    pub manager_env_uid: Option<EnvironmentUID>
}

pub type VirtualPersonaValueResult = Result<VirtualPersonaValue, GenericError>;