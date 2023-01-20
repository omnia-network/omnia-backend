use candid::candid_method;
use ic_cdk::{
    api::{call::call, caller},
    print,
};
use omnia_types::{
    environment::{EnvironmentInfo, EnvironmentUID},
    user::UserProfile,
};

use crate::utils::get_database_principal;

#[ic_cdk_macros::update(name = "getProfile")]
#[candid_method(update, rename = "getProfile")]
async fn get_profile() -> UserProfile {
    let user_principal = caller();

    let (user_profile,): (UserProfile,) = call(
        get_database_principal(),
        "getUserProfile",
        (user_principal.to_string(),),
    )
    .await
    .unwrap();

    print(format!("User profile: {:?}", user_profile));

    user_profile
}

#[ic_cdk_macros::update(name = "setEnvironment")]
#[candid_method(update, rename = "setEnvironment")]
async fn set_environment(env_uid: EnvironmentUID) -> EnvironmentInfo {
    let user_principal = caller();

    let (environment_info,): (EnvironmentInfo,) = call(
        get_database_principal(),
        "setUserInEnvironment",
        (user_principal.to_string(), env_uid),
    )
    .await
    .unwrap();

    print(format!("User in environment: {:?}", environment_info));

    environment_info
}

#[ic_cdk_macros::update(name = "resetEnvironment")]
#[candid_method(update, rename = "resetEnvironment")]
async fn reset_environment() -> EnvironmentInfo {
    let user_principal = caller();

    let (environment_info,): (EnvironmentInfo,) = call(
        get_database_principal(),
        "resetUserFromEnvironment",
        (user_principal.to_string(),),
    )
    .await
    .unwrap();

    print(format!("User not in environment: {:?}", environment_info));

    environment_info
}
