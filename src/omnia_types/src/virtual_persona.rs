use std::cmp::Ordering;

use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::{environment::EnvironmentUID, errors::GenericResult};

pub type VirtualPersonaPrincipalId = String;

pub type VirtualPersonaIp = String;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub struct VirtualPersonaIndex {
    pub principal_id: VirtualPersonaPrincipalId,
}

impl Ord for VirtualPersonaIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.principal_id.cmp(&other.principal_id)
    }
}

impl PartialOrd for VirtualPersonaIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct VirtualPersonaValue {
    pub virtual_persona_principal_id: VirtualPersonaPrincipalId,
    pub virtual_persona_ip: VirtualPersonaIp,
    pub user_env_uid: Option<EnvironmentUID>,
    pub manager_env_uid: Option<EnvironmentUID>,
}

pub type VirtualPersonaValueResult = GenericResult<VirtualPersonaValue>;
