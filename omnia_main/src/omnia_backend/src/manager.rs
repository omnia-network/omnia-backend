use ic_cdk::api;

use crate::INITIALIZED_GATEWAY_STORE;

type EnvironmentUid = String;

#[ic_cdk_macros::import(canister = "database")]
pub struct Database;



#[ic_cdk_macros::update(name = "createEnvironment")]
async fn create_environment(
    environment_creation_input: EnvironmentCreationInput
) -> Box<EnvironmentCreationResult> {

    let environment_manager_principal = api::caller();

    let environment_creation_result = Database::createNewEnvironment(
        environment_manager_principal.to_string(),
        Box::new(environment_creation_input)
    ).await.0;

    ic_cdk::print(format!("Created new environment: {:?}", environment_creation_result));

    environment_creation_result
}



#[ic_cdk_macros::update(name = "initGateway")]
async fn init_gateway() -> String {

    let gateway_uuid = Database::generateUuid().await.0;
    ic_cdk::print(format!("Initialized gateway with UUID: {:?}", gateway_uuid));

    INITIALIZED_GATEWAY_STORE.with(|intialized_gateway_store| {
        intialized_gateway_store.borrow_mut().insert(gateway_uuid.clone());
    });

    gateway_uuid
}



#[ic_cdk_macros::update(name = "registerGateway")]
async fn register_gateway(
    gateway_registration_input: GatewayRegistrationInput
) -> Option<Box<GatewayRegistrationResult>> {

    let environment_manager_principal = api::caller();

    let is_initialized = INITIALIZED_GATEWAY_STORE.with(|intialized_gateway_store| {
        let mut store = intialized_gateway_store.borrow_mut();
        match store.contains(&gateway_registration_input.gateway_uid) {
            true => store.remove(&gateway_registration_input.gateway_uid),
            false => false
        }
    });

    if is_initialized {
        let gateway_registration_result = Database::registerGatewayInEnvironment(
            environment_manager_principal.to_string(),
            Box::new(gateway_registration_input)
        ).await.0;
        
        ic_cdk::print(format!("Registered gateway: {:?}", gateway_registration_result));
        
        return Some(gateway_registration_result);
    }

    ic_cdk::print("Could not register gateway as it is not initialized");
    None
}



#[ic_cdk_macros::update(name = "getGateways")]
async fn get_gateways(
    environment_uid: EnvironmentUid,
) -> Option<Box<RegisteredGatewaysInfo>> {
    match Database::getGatewaysInEnvironment(environment_uid.clone()).await.0 {
        Some(gateways) => {
            ic_cdk::print(format!("Registered gateways: {:?}", gateways));
            Some(gateways)
        },
        None => {
            ic_cdk::print(format!("Environmnent: {:?} does not exist", environment_uid));
            None
        }
    }
}



#[ic_cdk_macros::update(name = "registerDevice")]
async fn register_device(
    device_registration_input: DeviceRegistrationInput
) -> Box<DeviceRegistrationResult> {

    let environment_manager_principal = api::caller();

    let device_registration_result = Database::registerDeviceInEnvironment(
        environment_manager_principal.to_string(),
        Box::new(device_registration_input)
    ).await.0;

    ic_cdk::print(format!("Registered device: {:?}", device_registration_result));

    device_registration_result
}



#[ic_cdk_macros::update(name = "getDevices")]
async fn get_devices(
    environment_uid: EnvironmentUid,
) -> Option<Box<RegisteredDevicesInfo>> {
    match Database::getDevicesInEnvironment(environment_uid.clone()).await.0 {
        Some(devices) => {
            ic_cdk::print(format!("Registered devices: {:?}", devices));
            Some(devices)
        },
        None => {
            ic_cdk::print(format!("Environmnent: {:?} does not exist", environment_uid));
            None
        }
    }
}