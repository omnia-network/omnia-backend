use candid::{Principal, CandidType, Deserialize};
use ic_cdk::api::{self, call::ManualReply};

// #[ic_cdk_macros::import(canister = "user_profile_backend")]
// struct UserProfileCanister;

// #[ic_cdk_macros::update]
// async fn set_environment_uid(uid: String) -> String {
//     let principal = api::caller();
//     ic_cdk::print(format!("User: {:?} is in environment with ID: {}", principal, uid));
//     UserProfileCanister::updateProfile(principal.to_string(), Box::new(Profile {
//         name: String::from("Massimo"),
//         description: String::from("omnia"),
//         keywords: vec![],
//     })).await;
//     uid
// }

// #[ic_cdk_macros::update(name = "whoami")]
// async fn whoami() -> Principal {
//     let principal = api::caller();
//     if UserProfileCanister::userIsInEnvironment(principal.to_string()).await.0 {
//         ic_cdk::print(format!("User: {:?} is already in environment", principal));
//     }
//     else {
//         ic_cdk::print(format!("User: {:?} is not in environment yet", principal));
//     }
//     principal
// }

// #[ic_cdk_macros::update(name = "getSelf")]
// async fn get_self() -> Box<Profile> {
//     UserProfileCanister::getSelf(api::caller().to_string()).await.0
// }

#[ic_cdk_macros::import(canister = "environments_db")]
struct EnvironmentsDatabaseCanister;

#[ic_cdk_macros::update(name = "setUserInEnvironment")]
fn set_user_in_environment(env_uid: String) -> ManualReply<EnvironmentInfo> {
    // TODO: add user to environment
    ManualReply::one(EnvironmentInfo {
        env_name: String::from("Example environment"),
        env_uid,
    })
}

#[ic_cdk_macros::update(name = "registerEnvironment")]
async fn register_environment(
    environment_registration_input: EnvironmentRegistrationInput
) -> Box<EnvironmentRegistrationResult> {

    let environment_manager_principal_id = api::caller().to_string();

    let environment_registration_result = EnvironmentsDatabaseCanister::InitializeNewEnvironment(
        environment_manager_principal_id,
        Box::new(environment_registration_input)
    ).await.0;

    ic_cdk::print(format!("Initialized environment: {:?}", environment_registration_result));

    environment_registration_result
}