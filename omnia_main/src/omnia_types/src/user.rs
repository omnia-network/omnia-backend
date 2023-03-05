use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::environment::EnvironmentUID;

pub type PrincipalId = String;
pub type Ip = String;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct VirtualPersona {
    pub virtual_persona_principal_id: PrincipalId,
    pub virtual_persona_ip: Ip,
    pub user_env_uid: Option<EnvironmentUID>,
    pub manager_env_uid: Option<EnvironmentUID>
}