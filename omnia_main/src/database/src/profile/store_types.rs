use candid::CandidType;
use omnia_types::{environment::EnvironmentUID, user::PrincipalId};

#[derive(Clone, Debug, CandidType)]
pub struct StoredUserProfile {
    pub user_principal_id: PrincipalId,
    pub environment_uid: Option<EnvironmentUID>,
}
