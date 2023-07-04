use candid::candid_method;
use ic_cdk::{
    api::{call::call, caller},
    print,
};
use omnia_types::{
    environment::EnvironmentInfoResult, http::IpChallengeNonce,
    virtual_persona::VirtualPersonaValueResult,
};

use crate::utils::get_database_principal;

#[ic_cdk_macros::update(name = "getProfile")]
#[candid_method(update, rename = "getProfile")]
async fn get_profile(nonce: IpChallengeNonce) -> VirtualPersonaValueResult {
    let virtual_persona_principal = caller();

    match call(
        get_database_principal(),
        "get_virtual_persona",
        (nonce, virtual_persona_principal.to_string()),
    )
    .await
    .unwrap()
    {
        (Ok(virtual_persona),) => {
            print(format!("User profile: {:?}", virtual_persona));
            Ok(virtual_persona)
        }
        (Err(e),) => Err(e),
    }
}

#[ic_cdk_macros::update(name = "setEnvironment")]
#[candid_method(update, rename = "setEnvironment")]
async fn set_environment(nonce: IpChallengeNonce) -> EnvironmentInfoResult {
    let virtual_persona_principal = caller();

    let (environment_info,): (EnvironmentInfoResult,) = call(
        get_database_principal(),
        "set_user_in_environment",
        (virtual_persona_principal.to_string(), nonce),
    )
    .await
    .unwrap();

    print(format!("User in environment: {:?}", environment_info));

    environment_info
}

#[ic_cdk_macros::update(name = "resetEnvironment")]
#[candid_method(update, rename = "resetEnvironment")]
async fn reset_environment(nonce: IpChallengeNonce) -> EnvironmentInfoResult {
    let virtual_persona_principal = caller();

    let (environment_info,): (EnvironmentInfoResult,) = call(
        get_database_principal(),
        "reset_user_from_environment",
        (virtual_persona_principal.to_string(), nonce),
    )
    .await
    .unwrap();

    print(format!("User not in environment: {:?}", environment_info));

    environment_info
}
