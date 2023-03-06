use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::environment::EnvironmentUID;

pub type VirtualPersonaPrincipalId = String;
pub type VirtualPersonaIp = String;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct VirtualPersona {
    pub virtual_persona_principal_id: VirtualPersonaPrincipalId,
    pub virtual_persona_ip: VirtualPersonaIp,
    pub user_env_uid: Option<EnvironmentUID>,
    pub manager_env_uid: Option<EnvironmentUID>
}