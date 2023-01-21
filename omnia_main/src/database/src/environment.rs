use candid::{candid_method, CandidType, Deserialize};
use ic_cdk::print;
use ic_cdk_macros::update;
use omnia_types::{
    device::{DeviceInfo, DeviceRegistrationInput, DeviceRegistrationResult, DeviceUID},
    environment::{EnvironmentCreationInput, EnvironmentCreationResult, EnvironmentUID},
    gateway::{GatewayInfo, GatewayRegistrationInput, GatewayRegistrationResult, GatewayUID},
    user::PrincipalId,
};
use serde::Serialize;
use std::collections::BTreeMap;

use crate::{uuid::generate_local_uuid, STATE};

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct StoredDeviceInfo {
    pub device_name: String,
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct StoredGatewayInfo {
    pub gateway_name: String,
    pub devices: BTreeMap<DeviceUID, StoredDeviceInfo>,
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct StoredEnvironmentInfo {
    pub env_name: String,
    pub env_gateways: BTreeMap<GatewayUID, StoredGatewayInfo>,
    pub env_manager_principal_id: PrincipalId,
}

#[update(name = "createNewEnvironment")]
#[candid_method(update, rename = "createNewEnvironment")]
fn create_new_environment(
    environment_manager_principal_id: PrincipalId,
    environment_creation_input: EnvironmentCreationInput,
) -> EnvironmentCreationResult {
    print(format!(
        "Creating new environment: {:?} managed by: {:?}",
        environment_creation_input, environment_manager_principal_id
    ));

    let environment_uid = generate_local_uuid();
    print(format!("Environment UID: {:?}", environment_uid));

    STATE.with(|state| {
        state.borrow_mut().environments.insert(
            environment_uid.clone(),
            StoredEnvironmentInfo {
                env_name: environment_creation_input.env_name.clone(),
                env_gateways: BTreeMap::new(),
                env_manager_principal_id: environment_manager_principal_id,
            },
        );
    });

    let environment_creation_result = EnvironmentCreationResult {
        env_name: environment_creation_input.env_name,
        env_uid: environment_uid,
    };

    environment_creation_result
}

#[update(name = "registerGatewayInEnvironment")]
#[candid_method(update, rename = "registerGatewayInEnvironment")]
fn register_gateway_in_environment(
    environment_manager_principal_id: PrincipalId,
    gateway_registration_input: GatewayRegistrationInput,
) -> Result<GatewayRegistrationResult, ()> {
    STATE.with(|state| {
        let mut mutable_state = state.borrow_mut();
        match mutable_state
            .environments
            .get_mut(&gateway_registration_input.env_uid)
        {
            Some(environment_info) => {
                print(format!(
                    "Registering gateway {:?} in environment with UID: {:?} managed by: {:?}",
                    gateway_registration_input.gateway_uid,
                    gateway_registration_input.env_uid,
                    environment_manager_principal_id
                ));

                environment_info.env_gateways.insert(
                    gateway_registration_input.gateway_uid.clone(),
                    StoredGatewayInfo {
                        gateway_name: gateway_registration_input.gateway_name.clone(),
                        devices: BTreeMap::new(),
                    },
                );

                print(format!("Updated environment: {:?}", environment_info));

                Ok(GatewayRegistrationResult {
                    gateway_name: gateway_registration_input.gateway_name,
                    gateway_uid: gateway_registration_input.gateway_uid,
                })
            }
            None => panic!("Environment does not exist"),
        }
    })
}

#[update(name = "registerDeviceInEnvironment")]
#[candid_method(update, rename = "registerDeviceInEnvironment")]
fn register_device_in_environment(
    environment_manager_principal_id: PrincipalId,
    device_registration_input: DeviceRegistrationInput,
) -> Result<DeviceRegistrationResult, ()> {
    STATE.with(|state| {
        let mut mutable_state = state.borrow_mut();
        match mutable_state
            .environments
            .get_mut(&device_registration_input.env_uid)
        {
            Some(environment_info) => {
                match environment_info
                    .env_gateways
                    .get_mut(&device_registration_input.gateway_uid)
                {
                    Some(gateway_info) => {
                        let device_uid = generate_local_uuid();
                        print(format!("Device UID: {:?}", device_uid));

                        gateway_info.devices.insert(
                            device_uid.clone(),
                            StoredDeviceInfo {
                                device_name: device_registration_input.device_name.clone(),
                            },
                        );

                        print(format!(
                            "Updated environment: {:?} managed by: {:?}",
                            environment_info, environment_manager_principal_id
                        ));

                        let device_registration_result = DeviceRegistrationResult {
                            device_name: device_registration_input.device_name,
                            device_uid,
                            gateway_uid: device_registration_input.gateway_uid.clone(),
                        };

                        Ok(device_registration_result)
                    }
                    None => panic!("Gateway does not exist in environment"),
                }
            }
            None => panic!("Environment does not exist"),
        }
    })
}

#[update(name = "getGatewaysInEnvironment")]
#[candid_method(update, rename = "getGatewaysInEnvironment")]
fn get_gateways_in_environment(environment_uid: EnvironmentUID) -> Result<Vec<GatewayInfo>, ()> {
    STATE.with(
        |state| match state.borrow().environments.get(&environment_uid) {
            Some(environment_info) => {
                let mut registered_gateways: Vec<GatewayInfo> = vec![];
                for (uuid, info) in environment_info.env_gateways.clone() {
                    let gateway_info = GatewayInfo {
                        gateway_name: info.gateway_name,
                        gateway_uid: uuid,
                    };
                    registered_gateways.push(gateway_info);
                }
                Ok(registered_gateways)
            }
            None => {
                print(format!(
                    "Environmnent: {:?} does not exist",
                    environment_uid
                ));
                Err(())
            }
        },
    )
}

#[update(name = "getDevicesInEnvironment")]
#[candid_method(update, rename = "getDevicesInEnvironment")]
fn get_devices_in_environment(environment_uid: EnvironmentUID) -> Result<Vec<DeviceInfo>, ()> {
    STATE.with(
        |state| match state.borrow().environments.get(&environment_uid) {
            Some(environment_info) => {
                let registered_devices = environment_info.env_gateways.iter().fold(
                    Vec::new(),
                    |mut registered_devices, (gateway_uid, gateway_info)| {
                        for (uuid, info) in gateway_info.devices.clone() {
                            let device_info = DeviceInfo {
                                device_name: info.device_name,
                                device_uid: uuid,
                                gateway_uid: gateway_uid.clone(),
                            };
                            registered_devices.push(device_info);
                        }
                        registered_devices
                    },
                );
                Ok(registered_devices)
            }
            None => {
                print(format!(
                    "Environmnent: {:?} does not exist",
                    environment_uid
                ));
                Err(())
            }
        },
    )
}
