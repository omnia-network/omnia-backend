use candid::CandidType;

type PrincipalId = String;
type EnvironmentUID = u32;

#[derive(Clone, Debug, CandidType)]
pub struct UserProfile {
    pub user_principal_id: PrincipalId,
    pub environment_uid: Option<EnvironmentUID>,
}
