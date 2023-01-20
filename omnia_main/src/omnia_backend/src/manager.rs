use ic_cdk::api;
use omnia_types::{
    device::{DeviceRegistrationInput, DeviceRegistrationResult, RegisteredDevicesInfo},
    environment::{EnvironmentCreationInput, EnvironmentCreationResult, EnvironmentUID},
    gateway::{GatewayRegistrationInput, GatewayRegistrationResult, RegisteredGatewaysInfo},
};

use crate::{utils::get_database_principal, stores::INITIALIZED_GATEWAY_STORE};

#[ic_cdk_macros::update(name = "createEnvironment")]
async fn create_environment(
    environment_creation_input: EnvironmentCreationInput,
) -> EnvironmentCreationResult {
    let environment_manager_principal = api::caller();

    let (environment_creation_result,): (EnvironmentCreationResult,) = api::call::call(
        get_database_principal(),
        "createNewEnvironment",
        (
            environment_manager_principal.to_string(),
            Box::new(environment_creation_input),
        ),
    )
    .await
    .unwrap();

    ic_cdk::print(format!(
        "Created new environment: {:?}",
        environment_creation_result
    ));

    environment_creation_result
}

#[ic_cdk_macros::update(name = "initGateway")]
async fn init_gateway() -> String {
    let (gateway_uuid,): (String,) = api::call::call(get_database_principal(), "generateUuid", ())
        .await
        .unwrap();

    ic_cdk::print(format!("Initialized gateway with UUID: {:?}", gateway_uuid));

    INITIALIZED_GATEWAY_STORE.with(|intialized_gateway_store| {
        intialized_gateway_store
            .borrow_mut()
            .insert(gateway_uuid.clone());
    });

    gateway_uuid
}

#[ic_cdk_macros::update(name = "registerGateway")]
async fn register_gateway(
    gateway_registration_input: GatewayRegistrationInput,
) -> Option<GatewayRegistrationResult> {
    let environment_manager_principal = api::caller();

    let is_initialized = INITIALIZED_GATEWAY_STORE.with(|intialized_gateway_store| {
        let mut store = intialized_gateway_store.borrow_mut();
        match store.contains(&gateway_registration_input.gateway_uid) {
            true => store.remove(&gateway_registration_input.gateway_uid),
            false => false,
        }
    });

    if is_initialized {
        let (gateway_registration_result,): (GatewayRegistrationResult,) = api::call::call(
            get_database_principal(),
            "registerGatewayInEnvironment",
            (
                environment_manager_principal.to_string(),
                Box::new(gateway_registration_input),
            ),
        )
        .await
        .unwrap();

        ic_cdk::print(format!(
            "Registered gateway: {:?}",
            gateway_registration_result
        ));

        return Some(gateway_registration_result);
    }

    ic_cdk::print("Could not register gateway as it is not initialized");
    None
}

#[ic_cdk_macros::update(name = "getGateways")]
async fn get_gateways(environment_uid: EnvironmentUID) -> Option<RegisteredGatewaysInfo> {
    let (res,): (Option<RegisteredGatewaysInfo>,) = api::call::call(
        get_database_principal(),
        "getGatewaysInEnvironment",
        (environment_uid.clone(),),
    )
    .await
    .unwrap();

    match res {
        Some(gateways) => {
            ic_cdk::print(format!("Registered gateways: {:?}", gateways));
            Some(gateways)
        }
        None => {
            ic_cdk::print(format!(
                "Environmnent: {:?} does not exist",
                environment_uid
            ));
            None
        }
    }
}

#[ic_cdk_macros::update(name = "registerDevice")]
async fn register_device(
    device_registration_input: DeviceRegistrationInput,
) -> DeviceRegistrationResult {
    let environment_manager_principal = api::caller();

    let (device_registration_result,): (DeviceRegistrationResult,) = api::call::call(
        get_database_principal(),
        "registerDeviceInEnvironment",
        (
            environment_manager_principal.to_string(),
            Box::new(device_registration_input),
        ),
    )
    .await
    .unwrap();

    ic_cdk::print(format!(
        "Registered device: {:?}",
        device_registration_result
    ));

    device_registration_result
}

#[ic_cdk_macros::update(name = "getDevices")]
async fn get_devices(environment_uid: EnvironmentUID) -> Option<RegisteredDevicesInfo> {
    let (res,): (Option<RegisteredDevicesInfo>,) = api::call::call(
        get_database_principal(),
        "getDevicesInEnvironment",
        (environment_uid.clone(),),
    )
    .await
    .unwrap();

    match res {
        Some(devices) => {
            ic_cdk::print(format!("Registered devices: {:?}", devices));
            Some(devices)
        }
        None => {
            ic_cdk::print(format!(
                "Environmnent: {:?} does not exist",
                environment_uid
            ));
            None
        }
    }
}
