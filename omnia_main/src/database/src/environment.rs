use candid::candid_method;
use ic_cdk::print;
use ic_cdk_macros::{update, query};
use omnia_types::{
    device::{
        DeviceInfo, DeviceInfoResult, DeviceRegistrationInput, MultipleDeviceInfoResult, StoredDeviceInfo,
    },
    environment::{EnvironmentCreationInput, EnvironmentCreationResult, EnvironmentUID, Environment},
    gateway::{
        RegisteredGateway, RegisteredGatewayResult, GatewayRegistrationInput,
        MultipleRegisteredGatewayResult, StoredRegisteredGateway, GatewayPrincipalId,
    },
    user::VirtualPersonaPrincipalId, http::{CanisterCallNonce, RequesterInfo},
};
use std::collections::BTreeMap;

use crate::{uuid::generate_uuid, STATE};

#[update(name = "initGatewayWithIp")]
#[candid_method(update, rename = "initGatewayWithIp")]
async fn init_gateway_with_ip(nonce: CanisterCallNonce, gateway_principal_id: GatewayPrincipalId) -> Result<String, ()> {

    let requester_info_to_be_checked: Option<RequesterInfo> = STATE.with(|state| {
        state
            .borrow_mut()
            .initialized_nonce_to_ip
            .remove(&nonce)
    });

    print(format!("Requester info to be checked: {:?}", requester_info_to_be_checked));

    match requester_info_to_be_checked {
        Some(gateway_request_info) => {        
            STATE.with(|state| {
                state
                    .borrow_mut()
                    .initialized_gateways
                    .insert(gateway_request_info.requester_ip, gateway_principal_id.clone());
            });
            Ok(gateway_principal_id)

        },
        None => {
            Err(())
        }
    }
}

#[update(name = "getInitializedGatewaysByIp")]
#[candid_method(update, rename = "getInitializedGatewaysByIp")]
async fn get_initialized_gateways_by_ip(nonce: CanisterCallNonce) -> Result<Vec<GatewayPrincipalId>, ()> {

    let requester_info_to_be_checked: Option<RequesterInfo> = STATE.with(|state| {
        state
            .borrow_mut()
            .initialized_nonce_to_ip
            .remove(&nonce)
    });

    print(format!("Requester info to be checked: {:?}", requester_info_to_be_checked));

    match requester_info_to_be_checked {
        Some(virtual_persona_request_info) => {
            STATE.with(|state| {
                match state
                    .borrow()
                    .initialized_gateways
                    .get(&virtual_persona_request_info.requester_ip) 
                {
                    Some(gateway_uid) => {
                        Ok(vec![gateway_uid.to_owned()])
                    },
                    None => Ok(vec![])
                }
            })
        },
        None => {
            Err(())
        }
    }
}

#[update(name = "createNewEnvironment")]
#[candid_method(update, rename = "createNewEnvironment")]
async fn create_new_environment(
    environment_manager_principal_id: VirtualPersonaPrincipalId,
    environment_creation_input: EnvironmentCreationInput,
) -> EnvironmentCreationResult {
    print(format!(
        "Creating new environment: {:?} managed by: {:?}",
        environment_creation_input, environment_manager_principal_id
    ));

    let environment_uid = generate_uuid().await;
    print(format!("Environment UID: {:?}", environment_uid));

    STATE.with(|state| {
        state.borrow_mut().environments.insert(
            environment_uid.clone(),
            Environment {
                env_name: environment_creation_input.env_name.clone(),
                env_ip: None,
                env_users_principals_ids: vec![],
                env_gateway_principal_ids: vec![],
                env_manager_principal_id: environment_manager_principal_id,
            },
        );
    });

    let environment_creation_result = EnvironmentCreationResult {
        env_name: environment_creation_input.env_name,
        env_uid: environment_uid,
    };

    print(format!(
        "Created new environment: {:?}",
        environment_creation_result
    ));

    environment_creation_result
}

#[update(name = "registerGatewayInEnvironment")]
#[candid_method(update, rename = "registerGatewayInEnvironment")]
fn register_gateway_in_environment(
    nonce: CanisterCallNonce,
    environment_manager_principal_id: VirtualPersonaPrincipalId,
    gateway_registration_input: GatewayRegistrationInput,
) -> RegisteredGatewayResult {

    let requester_info_to_be_checked: Option<RequesterInfo> = STATE.with(|state| {
        state
            .borrow_mut()
            .initialized_nonce_to_ip
            .remove(&nonce)
    });

    match requester_info_to_be_checked {
        Some(virtual_persona_request_info) => {
            let gateway_principal_id_option = STATE.with(|state| {
                state.borrow_mut()
                    .initialized_gateways
                    .remove(&virtual_persona_request_info.requester_ip)
            });
            match gateway_principal_id_option {
                Some(gateway_principal_id) => {
                    let registered_gateway_result = STATE.with(|state| {
                        match state.borrow_mut()
                            .environments
                            .get_mut(&gateway_registration_input.env_uid) 
                        {
                            Some(environment) => {
                                print(format!(
                                    "Registering gateway in environment with UID: {:?} managed by: {:?}",
                                    gateway_registration_input.env_uid,
                                    environment_manager_principal_id
                                ));

                                let registered_gateway = RegisteredGateway {
                                    gateway_name: gateway_registration_input.gateway_name,
                                    gateway_ip: virtual_persona_request_info.requester_ip,
                                    env_uid: gateway_registration_input.env_uid,
                                };

                                // add principal ID of registered Gateway to Environment
                                environment.env_gateway_principal_ids.push(gateway_principal_id.clone());
                                print(format!("Updated environment: {:?}", environment));
                                Ok(registered_gateway)
                            }
                            None => {
                                let err = format!(
                                    "Environment with uid {:?} does not exist",
                                    gateway_registration_input.env_uid
                                );
                
                                print(err.as_str());
                                Err(err)
                            }
                        }
                    });
                    match registered_gateway_result {
                        Ok(registered_gateway) => {
                            STATE.with(|state| {
                                state.borrow_mut()
                                    .registered_gateways
                                    .insert(gateway_principal_id, registered_gateway.clone());
                            });
                            Ok(registered_gateway)
                        },
                        Err(e) => Err(e),
                    }
                    
                },
                None => {
                    let err = format!(
                        "Gateway with IP {:?} has not been initialized",
                        virtual_persona_request_info.requester_ip
                    );

                    print(err.as_str());
                    Err(err)
                }
            }
        },
        None => {
            let err = format!(
                "Did not receive http request with nonce {:?} before canister call",
                nonce
            );

            print(err.as_str());
            Err(err)
        },
    }
}

// #[update(name = "registerDeviceInEnvironment")]
// #[candid_method(update, rename = "registerDeviceInEnvironment")]
// async fn register_device_in_environment(
//     environment_manager_principal_id: VirtualPersonaPrincipalId,
//     device_registration_input: DeviceRegistrationInput,
// ) -> DeviceInfoResult {
//     let device_uid = generate_uuid().await;

//     STATE.with(|state| {
//         let mut mutable_state = state.borrow_mut();
//         match mutable_state
//             .environments
//             .get_mut(&device_registration_input.env_uid)
//         {
//             Some(environment_info) => {
//                 match environment_info
//                     .env_gateways
//                     .get_mut(&device_registration_input.gateway_uid)
//                 {
//                     Some(gateway_info) => {
//                         print(format!("Device UID: {:?}", device_uid));

//                         gateway_info.devices.insert(
//                             device_uid.clone(),
//                             StoredDeviceInfo {
//                                 device_name: device_registration_input.device_name.clone(),
//                             },
//                         );

//                         print(format!(
//                             "Updated environment: {:?} managed by: {:?}",
//                             environment_info, environment_manager_principal_id
//                         ));

//                         let device_registration_result = DeviceInfo {
//                             device_name: device_registration_input.device_name,
//                             device_uid,
//                             gateway_uid: device_registration_input.gateway_uid.clone(),
//                         };

//                         Ok(device_registration_result)
//                     },
//                     None => {
//                         let err = format!(
//                             "Gateway with uid {:?} does not exist in environment",
//                             device_registration_input.gateway_uid
//                         );

//                         print(err.as_str());
//                         Err(err)
//                     }
//                 }
//             },
//             None => {
//                 let err = format!(
//                     "Environment with uid {:?} does not exist",
//                     device_registration_input.env_uid
//                 );

//                 print(err.as_str());
//                 Err(err)
//             }
//         }
//     })
// }

// #[update(name = "getGatewaysInEnvironment")]
// #[candid_method(update, rename = "getGatewaysInEnvironment")]
// fn get_gateways_in_environment(environment_uid: EnvironmentUID) -> MultipleRegisteredGatewayResult {
//     STATE.with(
//         |state| match state.borrow().environments.get(&environment_uid) {
//             Some(environment_info) => {
//                 let mut registered_gateways: Vec<RegisteredGateway> = vec![];
//                 for (uuid, info) in environment_info.env_gateways.clone() {
//                     let gateway_info = RegisteredGateway {
//                         gateway_name: info.gateway_name,
//                         gateway_uid: uuid,
//                     };
//                     registered_gateways.push(gateway_info);
//                 }
//                 Ok(registered_gateways)
//             }
//             None => {
//                 let err = format!("Environmnent: {:?} does not exist", environment_uid);

//                 print(err.as_str());
//                 Err(err)
//             }
//         },
//     )
// }

// #[update(name = "getDevicesInEnvironment")]
// #[candid_method(update, rename = "getDevicesInEnvironment")]
// fn get_devices_in_environment(environment_uid: EnvironmentUID) -> MultipleDeviceInfoResult {
//     STATE.with(
//         |state| match state.borrow().environments.get(&environment_uid) {
//             Some(environment_info) => {
//                 let registered_devices = environment_info.env_gateways.iter().fold(
//                     Vec::new(),
//                     |mut registered_devices, (gateway_uid, gateway_info)| {
//                         for (uuid, info) in gateway_info.devices.clone() {
//                             let device_info = DeviceInfo {
//                                 device_name: info.device_name,
//                                 device_uid: uuid,
//                                 gateway_uid: gateway_uid.clone(),
//                             };
//                             registered_devices.push(device_info);
//                         }
//                         registered_devices
//                     },
//                 );
//                 Ok(registered_devices)
//             }
//             None => {
//                 let err = format!("Environmnent: {:?} does not exist", environment_uid);

//                 print(err.as_str());
//                 Err(err)
//             }
//         },
//     )
// }
