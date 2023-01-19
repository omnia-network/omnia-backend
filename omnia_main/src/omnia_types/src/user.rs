use candid::{CandidType, Deserialize};

use crate::environment::EnvironmentUID;

pub type PrincipalId = String;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct UserProfile {
    pub user_principal_id: PrincipalId,
    pub environment_uid: Option<EnvironmentUID>,
}
