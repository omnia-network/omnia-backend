use candid::candid_method;
use ic_cdk::{
    api::{call::call, caller},
    print,
};
use omnia_types::{
    environment::EnvironmentInfoResult,
    virtual_persona::VirtualPersonaValue, http::IpChallengeNonce,
};

use crate::utils::get_database_principal;

#[ic_cdk_macros::update(name = "getProfile")]
#[candid_method(update, rename = "getProfile")]
async fn get_profile(nonce: IpChallengeNonce) -> Result<VirtualPersonaValue, ()> {
    let virtual_persona_principal = caller();

    match call(
        get_database_principal(),
        "getVirtualPersona",
        (nonce, virtual_persona_principal.to_string(),),
    ).await.unwrap() {
        (Ok(virtual_persona),) => {
            print(format!("User profile: {:?}", virtual_persona));
            Ok(virtual_persona)
        },
        (Err(()), ) => Err(())
    }
}

// #[ic_cdk_macros::update(name = "setEnvironment")]
// #[candid_method(update, rename = "setEnvironment")]
// async fn set_environment(nonce: CanisterCallNonce) -> EnvironmentInfoResult {
//     let virtual_persona_principal = caller();

//     let (environment_info,): (EnvironmentInfoResult,) = call(
//         get_database_principal(),
//         "setUserInEnvironment",
//         (virtual_persona_principal.to_string(), nonce, ),
//     )
//     .await
//     .unwrap();

//     print(format!("User in environment: {:?}", environment_info));

//     environment_info
// }

// #[ic_cdk_macros::update(name = "resetEnvironment")]
// #[candid_method(update, rename = "resetEnvironment")]
// async fn reset_environment(nonce: CanisterCallNonce) -> EnvironmentInfoResult {
//     let virtual_persona_principal = caller();

//     let (environment_info,): (EnvironmentInfoResult,) = call(
//         get_database_principal(),
//         "resetUserFromEnvironment",
//         (virtual_persona_principal.to_string(), nonce, ),
//     )
//     .await
//     .unwrap();

//     print(format!("User not in environment: {:?}", environment_info));

//     environment_info
// }
