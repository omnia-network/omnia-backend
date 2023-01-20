use ic_cdk::api;
use omnia_types::{
    environment::{EnvironmentInfo, EnvironmentUID},
    user::UserProfile,
};

use crate::{utils::get_database_principal};

#[ic_cdk_macros::update(name = "getProfile")]
async fn get_profile() -> UserProfile {
    let user_principal = api::caller();
    
    let (user_profile,): (UserProfile,) = api::call::call(
        get_database_principal(),
        "getUserProfile",
        (user_principal.to_string(),),
    )
    .await
    .unwrap();

    ic_cdk::print(format!("User profile: {:?}", user_profile));

    user_profile
}

#[ic_cdk_macros::update(name = "setEnvironment")]
async fn set_environment(env_uid: EnvironmentUID) -> EnvironmentInfo {
    let user_principal = api::caller();

    let (environment_info,): (EnvironmentInfo,) = api::call::call(
        get_database_principal(),
        "setUserInEnvironment",
        (user_principal.to_string(),env_uid,),
    )
    .await
    .unwrap();

    ic_cdk::print(format!("User in environment: {:?}", environment_info));

    environment_info
}

#[ic_cdk_macros::update(name = "resetEnvironment")]
async fn reset_environment() -> EnvironmentInfo {
    let user_principal = api::caller();

    let (environment_info,): (EnvironmentInfo,) = api::call::call(
        get_database_principal(),
        "resetUserFromEnvironment",
        (user_principal.to_string(),),
    )
    .await
    .unwrap();

    ic_cdk::print(format!("User not in environment: {:?}", environment_info));

    environment_info
}
