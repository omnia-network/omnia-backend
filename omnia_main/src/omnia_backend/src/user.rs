use candid::Principal;
use ic_cdk::api;
// use ic_cdk::export::candid::encode_args;
use omnia_types::{
    environment::{EnvironmentInfo, EnvironmentUID},
    user::UserProfile,
};

// #[ic_cdk_macros::init]
// async fn initialize() {
//     // let canister_args_result = encode_args(("MyName", "MN", 18 as u64, 0 as u64));
//     // let canister_args: Vec<u8> = match canister_args_result {
//     //     Ok(res) => {
//     //         ic_cdk::print(format!("{:?}", res));
//     //         Vec::new()
//     //     },
//     //     _ => Vec::new(),
//     // };
//     let call_arg = ic_cdk::api::call::arg_data::<(Option<String>,)>().0;
//     ic_cdk::print(format!("Init: {:?}", call_arg));
// }

// #[ic_cdk_macros::import(canister = "database")]
// pub struct Database;

#[ic_cdk_macros::update(name = "getProfile")]
async fn get_profile() -> UserProfile {
    let user_principal = api::caller();

    let remote_principal: Principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let (user_profile,): (UserProfile,) = api::call::call(
        remote_principal,
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

    let remote_principal: Principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let (environment_info,): (EnvironmentInfo,) = api::call::call(
        remote_principal,
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

    let remote_principal: Principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let (environment_info,): (EnvironmentInfo,) = api::call::call(
        remote_principal,
        "resetUserFromEnvironment",
        (user_principal.to_string(),),
    )
    .await
    .unwrap();

    ic_cdk::print(format!("User not in environment: {:?}", environment_info));

    environment_info
}
