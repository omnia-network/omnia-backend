use candid::candid_method;
use ic_cdk::{
    api::{call::call, caller},
    print,
};
use ic_cdk_macros::update;
use omnia_types::{
    device::{DeviceInfoResult, DeviceRegistrationInput, MultipleDeviceInfoResult},
    environment::{EnvironmentCreationInput, EnvironmentCreationResult, EnvironmentUID},
    gateway::{RegisteredgatewayResult, GatewayRegistrationInput, MultipleRegisteredgatewayResult, GatewayUID}, http::CanisterCallNonce, user::VirtualPersona,
};

use crate::utils::get_database_principal;

#[update(name = "createEnvironment")]
#[candid_method(update, rename = "createEnvironment")]
async fn create_environment(
    environment_creation_input: EnvironmentCreationInput,
) -> Result<EnvironmentCreationResult, ()> {
    let environment_manager_principal = caller();

    let (virtual_persona_option, ): (Option<VirtualPersona>, ) = call(
        get_database_principal(),
        "getVirtualPersonaIfExists",
        (environment_manager_principal,),
    ).await.unwrap();
    match virtual_persona_option {
        Some(_) => {
            let (environment_creation_result,): (EnvironmentCreationResult,) = call(
                get_database_principal(),
                "createNewEnvironment",
                (
                    environment_manager_principal.to_string(),
                    Box::new(environment_creation_input),
                ),
            )
            .await
            .unwrap();
        
            print(format!(
                "Created new environment: {:?}",
                environment_creation_result
            ));
        
            Ok(environment_creation_result)
        },
        None => {
            Err(())
        }
    }
}

#[update(name = "initGateway")]
#[candid_method(update, rename = "initGateway")]
async fn init_gateway(nonce: CanisterCallNonce) -> Result<String, ()> {
    let gateway_principal = caller();

    match call(get_database_principal(), "initGatewayWithIp", (
        nonce,
        gateway_principal.to_string(),
    ))
    .await
    .unwrap() {
        (Ok(gateway_uuid),) => {
            print(format!("Initialized gateway with UUID: {:?}", gateway_uuid));
            Ok(gateway_uuid)
        },
        (Err(()),) => Err(())
    }
}

#[update(name = "getInitializedGateways")]
#[candid_method(update, rename = "getInitializedGateways")]
async fn get_initialized_gateways(nonce: CanisterCallNonce) -> Result<Vec<GatewayUID>, ()> {
    
    let initialized_gateway_principals_result: Result<Vec<GatewayUID>, ()> = match call(get_database_principal(), "getInitializedGatewaysByIp", (nonce, ))
        .await
        .unwrap()
    {
        (Ok(initialized_gateway_principals),) => {
            print(format!("Initialized gateways in the local network have principals {:?}", initialized_gateway_principals));
            Ok(initialized_gateway_principals)
        },
        (Err(()),) => Err(())
    };
    initialized_gateway_principals_result
}

#[update(name = "registerGateway")]
#[candid_method(update, rename = "registerGateway")]
async fn register_gateway(
    nonce: CanisterCallNonce,
    gateway_registration_input: GatewayRegistrationInput,
) -> RegisteredgatewayResult {
    let environment_manager_principal = caller();

    let (gateway_registration_result,): (RegisteredgatewayResult,) = call(
        get_database_principal(),
        "registerGatewayInEnvironment",
        (
            nonce,
            environment_manager_principal.to_string(),
            Box::new(gateway_registration_input),
        ),
    )
    .await
    .unwrap();

    gateway_registration_result
}

// #[update(name = "getGateways")]
// #[candid_method(update, rename = "getGateways")]
// async fn get_gateways(environment_uid: EnvironmentUID) -> MultipleRegisteredgatewayResult {
//     let (res,): (MultipleRegisteredgatewayResult,) = call(
//         get_database_principal(),
//         "getGatewaysInEnvironment",
//         (environment_uid.clone(),),
//     )
//     .await
//     .unwrap();

//     res
// }

// #[update(name = "registerDevice")]
// #[candid_method(update, rename = "registerDevice")]
// async fn register_device(device_registration_input: DeviceRegistrationInput) -> DeviceInfoResult {
//     let environment_manager_principal = caller();

//     let (device_registration_result,): (DeviceInfoResult,) = call(
//         get_database_principal(),
//         "registerDeviceInEnvironment",
//         (
//             environment_manager_principal.to_string(),
//             Box::new(device_registration_input),
//         ),
//     )
//     .await
//     .unwrap();

//     device_registration_result
// }

// #[update(name = "getDevices")]
// #[candid_method(update, rename = "getDevices")]
// async fn get_devices(environment_uid: EnvironmentUID) -> MultipleDeviceInfoResult {
//     let (res,): (MultipleDeviceInfoResult,) = call(
//         get_database_principal(),
//         "getDevicesInEnvironment",
//         (environment_uid.clone(),),
//     )
//     .await
//     .unwrap();

//     res
// }
