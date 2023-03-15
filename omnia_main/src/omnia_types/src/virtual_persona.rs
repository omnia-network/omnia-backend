use std::cmp::Ordering;

use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::{environment::EnvironmentUID, MapEntry};

pub type VirtualPersonaPrincipalId = String;

pub type VirtualPersonaIp = String;

#[derive(Clone, CandidType, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
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

pub struct VirtualPersonaEntry {
    pub index: VirtualPersonaIndex,
    pub value: VirtualPersonaValue,
}

impl MapEntry for VirtualPersonaEntry {
    type MapIndex = VirtualPersonaIndex;
    type MapValue = VirtualPersonaValue;

    fn get_index(&self) ->  Self::MapIndex {
        self.index.clone()
    }

    fn get_value(&self) -> Self::MapValue {
        self.value.clone()
    }
}