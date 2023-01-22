use candid::candid_method;
use ic_cdk::{
    api::{call::call, caller},
    print,
};
use ic_cdk_macros::update;
use omnia_types::{
    device::{DeviceInfoResult, DeviceRegistrationInput, MultipleDeviceInfoResult},
    environment::{EnvironmentCreationInput, EnvironmentCreationResult, EnvironmentUID},
    gateway::{GatewayInfoResult, GatewayRegistrationInput, MultipleGatewayInfoResult},
};

use crate::utils::get_database_principal;

#[update(name = "createEnvironment")]
#[candid_method(update, rename = "createEnvironment")]
async fn create_environment(
    environment_creation_input: EnvironmentCreationInput,
) -> EnvironmentCreationResult {
    let environment_manager_principal = caller();

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

    environment_creation_result
}

#[update(name = "initGateway")]
#[candid_method(update, rename = "initGateway")]
async fn init_gateway() -> String {
    let (gateway_uuid,): (String,) = call(get_database_principal(), "initGateway", ())
        .await
        .unwrap();

    print(format!("Initialized gateway with UUID: {:?}", gateway_uuid));

    gateway_uuid
}

#[update(name = "registerGateway")]
#[candid_method(update, rename = "registerGateway")]
async fn register_gateway(
    gateway_registration_input: GatewayRegistrationInput,
) -> GatewayInfoResult {
    let environment_manager_principal = caller();

    let (gateway_registration_result,): (GatewayInfoResult,) = call(
        get_database_principal(),
        "registerGatewayInEnvironment",
        (
            environment_manager_principal.to_string(),
            Box::new(gateway_registration_input),
        ),
    )
    .await
    .unwrap();

    gateway_registration_result
}

#[update(name = "getGateways")]
#[candid_method(update, rename = "getGateways")]
async fn get_gateways(environment_uid: EnvironmentUID) -> MultipleGatewayInfoResult {
    let (res,): (MultipleGatewayInfoResult,) = call(
        get_database_principal(),
        "getGatewaysInEnvironment",
        (environment_uid.clone(),),
    )
    .await
    .unwrap();

    res
}

#[update(name = "registerDevice")]
#[candid_method(update, rename = "registerDevice")]
async fn register_device(device_registration_input: DeviceRegistrationInput) -> DeviceInfoResult {
    let environment_manager_principal = caller();

    let (device_registration_result,): (DeviceInfoResult,) = call(
        get_database_principal(),
        "registerDeviceInEnvironment",
        (
            environment_manager_principal.to_string(),
            Box::new(device_registration_input),
        ),
    )
    .await
    .unwrap();

    device_registration_result
}

#[update(name = "getDevices")]
#[candid_method(update, rename = "getDevices")]
async fn get_devices(environment_uid: EnvironmentUID) -> MultipleDeviceInfoResult {
    let (res,): (MultipleDeviceInfoResult,) = call(
        get_database_principal(),
        "getDevicesInEnvironment",
        (environment_uid.clone(),),
    )
    .await
    .unwrap();

    res
}
