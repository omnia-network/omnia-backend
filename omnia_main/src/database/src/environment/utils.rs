use ic_cdk::api::call::ManualReply;
use std::collections::BTreeMap;
use super::interface_types as InterfaceTypes;
use super::interface_types::GatewayInfo;
use super::interface_types::RegisteredGatewaysInfo;
use super::store_types as StoreTypes;
use super::ENVIRONMENT_STORE;
use crate::generate_local_uuid;

type PrincipalId = String;
type EnvironmentUID = String;



#[ic_cdk_macros::update(name = "createNewEnvironment", manual_reply = true)]
fn create_new_environment(
    environment_manager_principal_id: PrincipalId,
    environment_creation_input: InterfaceTypes::EnvironmentCreationInput
) -> ManualReply<InterfaceTypes::EnvironmentCreationResult> {

    ic_cdk::print(format!("Creating new environment: {:?} managed by: {:?}", environment_creation_input, environment_manager_principal_id));

    let environment_uid = generate_local_uuid();
    ic_cdk::print(format!("Environment UID: {:?}", environment_uid));

    ENVIRONMENT_STORE.with(|environment_store| {
        environment_store.borrow_mut().insert(
            environment_uid.clone(),
            StoreTypes::EnvironmentInfo {
                env_name: environment_creation_input.env_name.clone(),
                env_uid: environment_uid.clone(),
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
            ic_cdk::print(format!("Registering gateway {:?} in environment with UID: {:?} managed by: {:?}", gateway_registration_input.gateway_uid, gateway_registration_input.env_uid, environment_manager_principal_id));

            environment_info.env_gateways.insert(
                gateway_registration_input.gateway_uid.clone(),
                StoreTypes::GatewayInfo {
                    gateway_name: gateway_registration_input.gateway_name.clone(),
                    gateway_uid: gateway_registration_input.gateway_uid.clone(),
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
                gateway_uid: gateway_registration_input.gateway_uid,
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

                    let device_uid = generate_local_uuid();
                    ic_cdk::print(format!("Device UID: {:?}", device_uid));

                    gateway_info.devices.insert(
                        device_uid.clone(),
                        StoreTypes::DeviceInfo {
                            device_name: device_registration_input.device_name.clone(),
                        }
                    );

                    environment_info.env_gateways.insert(
                        device_registration_input.gateway_uid,
                        gateway_info.clone()
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
                        gateway_uid: gateway_info.gateway_uid.clone()
                    };

                    ManualReply::one(device_registration_result)
                },
                None => panic!("Gateway does not exist in environment"),
            }
        },
        None => panic!("Environment does not exist"),
    }
}



#[ic_cdk_macros::update(name = "getGatewaysInEnvironment", manual_reply = true)]
fn get_gateways_in_environment(
    environment_uid: EnvironmentUID,
) -> ManualReply<Option<RegisteredGatewaysInfo>> {
    let gateways = match get_environment_info_by_uid(&environment_uid) {
        Some(environment_info) => {
            let mut registered_gateways_info = RegisteredGatewaysInfo {
                registered_gateways: vec![],
            };
            for (_, info) in environment_info.env_gateways {
                let gateway_info = GatewayInfo {
                    gateway_name: info.gateway_name,
                    gateway_uid: info.gateway_uid,
                };
                registered_gateways_info.registered_gateways.insert(0, gateway_info);
            }
            Some(registered_gateways_info)
        },
        None => None
    };
    ManualReply::one(gateways)
}



pub fn get_environment_info_by_uid(environment_uid: &EnvironmentUID) -> Option<StoreTypes::EnvironmentInfo> {
    ENVIRONMENT_STORE.with(|environment_store| {
        match environment_store.borrow().get(environment_uid) {
            Some(mut environment_info) => Some(environment_info.to_owned()),
            None => None,
        }
    })
}