use ic_cdk::api::call::ManualReply;
use std::collections::BTreeMap;
use rand::Rng;
use super::interface_types as InterfaceTypes;
use super::store_types as StoreTypes;
use super::ENVIRONMENT_STORE;

type PrincipalId = String;
type EnvironmentUID = u32;



#[ic_cdk_macros::update(name = "createNewEnvironment", manual_reply = true)]
fn create_new_environment(
    environment_manager_principal_id: PrincipalId,
    environment_creation_input: InterfaceTypes::EnvironmentCreationInput
) -> ManualReply<InterfaceTypes::EnvironmentCreationResult> {

    ic_cdk::print(format!("Creating new environment: {:?} managed by: {:?}", environment_creation_input, environment_manager_principal_id));

    let environment_uid = rand::thread_rng().gen_range(0..100);
    ic_cdk::print(format!("Environment UID: {:?}", environment_uid));

    ENVIRONMENT_STORE.with(|environment_store| {
        environment_store.borrow_mut().insert(
            environment_uid,
            StoreTypes::EnvironmentInfo {
                env_name: environment_creation_input.env_name.clone(),
                env_uid: environment_uid,
                env_gateways: BTreeMap::new(),
                env_manager_principal_id: environment_manager_principal_id,
            }
        );
    });

    let environment_creation_result = InterfaceTypes::EnvironmentCreationResult {
        env_name: environment_creation_input.env_name,
        env_uid: environment_uid,
    };

    ManualReply::one(environment_creation_result)
}



#[ic_cdk_macros::update(name = "registerGatewayInEnvironment", manual_reply = true)]
fn register_gateway_in_environment(
    environment_manager_principal_id: PrincipalId,
    gateway_registration_input: InterfaceTypes::GatewayRegistrationInput
) -> ManualReply<InterfaceTypes::GatewayRegistrationResult> {

    match get_environment_info_by_uid(&gateway_registration_input.env_uid) {
        Some(mut environment_info) => {
            ic_cdk::print(format!("Registering gateway {:?} in environment with UID: {:?} managed by: {:?}", gateway_registration_input.gateway_name, gateway_registration_input.env_uid, environment_manager_principal_id));
        
            let gateway_uid = rand::thread_rng().gen_range(0..100);
            ic_cdk::print(format!("Gateway UID: {:?}", gateway_uid));

            environment_info.env_gateways.insert(
                gateway_uid,
                StoreTypes::GatewayInfo {
                    gateway_name: gateway_registration_input.gateway_name.clone(),
                    gateway_uid,
                    devices: BTreeMap::new(),
                }
            );

            ic_cdk::print(format!("Updated environment: {:?}", environment_info));

            ENVIRONMENT_STORE.with(|environment_store| {
                environment_store.borrow_mut().insert(
                    gateway_registration_input.env_uid,
                    environment_info
                )
            });

            let gateway_registration_result = InterfaceTypes::GatewayRegistrationResult {
                gateway_name: gateway_registration_input.gateway_name,
                gateway_uid,
            };

            ManualReply::one(gateway_registration_result)
        },
        None => panic!("Environment does not exist"),
    }
}



#[ic_cdk_macros::update(name = "registerDeviceInEnvironment", manual_reply = true)]
fn register_device_in_environment(
    environment_manager_principal_id: PrincipalId,
    device_registration_input: InterfaceTypes::DeviceRegistrationInput
) -> ManualReply<InterfaceTypes::DeviceRegistrationResult> {

    match get_environment_info_by_uid(&device_registration_input.env_uid) {
        Some(mut environment_info) => {

            match environment_info.env_gateways.remove(&device_registration_input.gateway_uid) {
                Some(mut gateway_info) => {

                    let device_uid = rand::thread_rng().gen_range(0..100);
                    ic_cdk::print(format!("Device UID: {:?}", device_uid));

                    gateway_info.devices.insert(
                        device_uid,
                        StoreTypes::DeviceInfo {
                            device_name: device_registration_input.device_name.clone(),
                        }
                    );

                    environment_info.env_gateways.insert(
                        device_registration_input.gateway_uid,
                        gateway_info
                    );

                    ic_cdk::print(format!("Updated environment: {:?} managed by: {:?}", environment_info, environment_manager_principal_id));

                    ENVIRONMENT_STORE.with(|environment_store| {
                        environment_store.borrow_mut().insert(
                            device_registration_input.env_uid,
                            environment_info
                        )
                    });

                    let device_registration_result = InterfaceTypes::DeviceRegistrationResult {
                        device_name: device_registration_input.device_name,
                        device_uid,
                    };

                    ManualReply::one(device_registration_result)
                },
                None => panic!("Gateway does not exist in environment"),
            }
        },
        None => panic!("Environment does not exist"),
    }
}



pub fn get_environment_info_by_uid(environment_uid: &EnvironmentUID) -> Option<StoreTypes::EnvironmentInfo> {
    ENVIRONMENT_STORE.with(|environment_store| {
        match environment_store.borrow().get(environment_uid) {
            Some(mut environment_info) => Some(environment_info.to_owned()),
            None => None,
        }
    })
}
