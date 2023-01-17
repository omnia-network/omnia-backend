use ic_cdk::api;

type EnvironmentUID = String;

#[ic_cdk_macros::import(canister = "database")]
pub struct Database;



#[ic_cdk_macros::update(name = "getProfile")]
async fn get_profile() -> Box<UserProfile> {
    let user_principal = api::caller();

    let user_profile = Database::getUserProfile(user_principal.to_string()).await.0;

    ic_cdk::print(format!("User profile: {:?}", user_profile));

    user_profile
}



#[ic_cdk_macros::update(name = "setEnvironment")]
async fn set_environment(env_uid: EnvironmentUID) -> Box<EnvironmentInfo> {
    let user_principal = api::caller();

    let environment_info = Database::setUserInEnvironment(user_principal.to_string(), env_uid).await.0;
    ic_cdk::print(format!("User in environment: {:?}", environment_info));

    environment_info
}



#[ic_cdk_macros::update(name = "resetEnvironment")]
async fn reset_environment() -> Box<EnvironmentInfo> {
    let user_principal = api::caller();

    let environment_info = Database::resetUserFromEnvironment(user_principal.to_string()).await.0;
    ic_cdk::print(format!("User not in environment: {:?}", environment_info));

    environment_info
}