use candid::candid_method;
use ic_cdk::{
    api::{call::call, caller},
    print,
};
use ic_cdk_macros::update;
use omnia_types::{
    device::{DeviceInfo, DeviceRegistrationInput, DeviceRegistrationResult},
    environment::{EnvironmentCreationInput, EnvironmentCreationResult, EnvironmentUID},
    gateway::{GatewayInfo, GatewayRegistrationInput, GatewayRegistrationResult},
};

use crate::{utils::get_database_principal, STATE};

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
    let (gateway_uuid,): (String,) = call(get_database_principal(), "generateUuid", ())
        .await
        .unwrap();

    print(format!("Initialized gateway with UUID: {:?}", gateway_uuid));

    STATE.with(|state| {
        state
            .borrow_mut()
            .gateways_uids
            .insert(gateway_uuid.clone());
    });

    gateway_uuid
}

#[update(name = "registerGateway")]
#[candid_method(update, rename = "registerGateway")]
async fn register_gateway(
    gateway_registration_input: GatewayRegistrationInput,
) -> Option<GatewayRegistrationResult> {
    let environment_manager_principal = caller();

    let is_initialized = STATE.with(|state| {
        let mut state = state.borrow_mut();
        match state
            .gateways_uids
            .contains(&gateway_registration_input.gateway_uid)
        {
            true => state
                .gateways_uids
                .remove(&gateway_registration_input.gateway_uid),
            false => false,
        }
    });

    if is_initialized {
        let (gateway_registration_result,): (GatewayRegistrationResult,) = call(
            get_database_principal(),
            "registerGatewayInEnvironment",
            (
                environment_manager_principal.to_string(),
                Box::new(gateway_registration_input),
            ),
        )
        .await
        .unwrap();

        print(format!(
            "Registered gateway: {:?}",
            gateway_registration_result
        ));

        return Some(gateway_registration_result);
    }

    print("Could not register gateway as it is not initialized");
    None
}

#[update(name = "getGateways")]
#[candid_method(update, rename = "getGateways")]
async fn get_gateways(environment_uid: EnvironmentUID) -> Vec<GatewayInfo> {
    let (res,): (Vec<GatewayInfo>,) = call(
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
async fn register_device(
    device_registration_input: DeviceRegistrationInput,
) -> DeviceRegistrationResult {
    let environment_manager_principal = caller();

    let (device_registration_result,): (DeviceRegistrationResult,) = call(
        get_database_principal(),
        "registerDeviceInEnvironment",
        (
            environment_manager_principal.to_string(),
            Box::new(device_registration_input),
        ),
    )
    .await
    .unwrap();

    print(format!(
        "Registered device: {:?}",
        device_registration_result
    ));

    device_registration_result
}

#[update(name = "getDevices")]
#[candid_method(update, rename = "getDevices")]
async fn get_devices(environment_uid: EnvironmentUID) -> Vec<DeviceInfo> {
    let (res,): (Vec<DeviceInfo>,) = call(
        get_database_principal(),
        "getDevicesInEnvironment",
        (environment_uid.clone(),),
    )
    .await
    .unwrap();

    res
}
