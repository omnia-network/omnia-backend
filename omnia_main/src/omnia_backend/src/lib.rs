use candid::Principal;
use ic_cdk::api;

#[ic_cdk_macros::import(canister = "user_profile_backend")]
struct UserProfileCanister;

#[ic_cdk_macros::update]
async fn set_environment_uid(uid: String) -> String {
    let principal = api::caller();
    ic_cdk::print(format!("User: {:?} is in environment with ID: {}", principal, uid));
    UserProfileCanister::updateProfile(principal.to_string(), Box::new(Profile {
        name: String::from("Massimo"),
        description: String::from("omnia"),
        keywords: vec![],
    })).await;
    uid
}

#[ic_cdk_macros::update(name = "whoami")]
async fn whoami() -> Principal {
    let principal = api::caller();
    if UserProfileCanister::userIsInEnvironment(principal.to_string()).await.0 {
        ic_cdk::print(format!("User: {:?} is already in environment", principal));
    }
    else {
        ic_cdk::print(format!("User: {:?} is not in environment yet", principal));
    }
    principal
}

#[ic_cdk_macros::update(name = "getSelf")]
async fn get_self() -> Box<Profile> {
    UserProfileCanister::getSelf(api::caller().to_string()).await.0
}

#[ic_cdk_macros::query]
fn get_device_uid() -> String {
    String::from("omnia_device_0")
}