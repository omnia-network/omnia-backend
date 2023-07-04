use candid::candid_method;
use ic_cdk::print;
use ic_cdk_macros::update;
use omnia_types::{
    device::{
        RegisteredDeviceIndex, RegisteredDeviceResult, RegisteredDeviceValue,
        RegisteredDevicesUidsResult,
    },
    environment::{
        EnvironmentCreationInput, EnvironmentCreationResult, EnvironmentIndex, EnvironmentUID,
        EnvironmentUidIndex, EnvironmentUidValue, EnvironmentValue,
    },
    errors::GenericResult,
    gateway::{
        GatewayPrincipalId, GatewayRegistrationInput, InitializedGatewayIndex,
        InitializedGatewayValue, MultipleRegisteredGatewayResult, RegisteredGatewayIndex,
        RegisteredGatewayResult, RegisteredGatewayValue,
    },
    http::IpChallengeNonce,
    updates::{
        PairingInfo, PairingPayload, UpdateIndex, UpdateValue, UpdateValueOption, UpdateValueResult,
    },
    virtual_persona::{VirtualPersonaIndex, VirtualPersonaPrincipalId},
};
use omnia_utils::net::{get_device_url, get_gateway_url};
use std::collections::BTreeMap;
use uuid::Uuid;

use crate::{utils::caller_is_omnia_backend, STATE};

#[update]
#[candid_method(update)]
async fn is_gateway_registered(gateway_principal_id: GatewayPrincipalId) -> bool {
    caller_is_omnia_backend();

    STATE.with(|state| {
        // check existance in registered gateways
        let registered_gateway_index = RegisteredGatewayIndex {
            principal_id: gateway_principal_id,
        };

        state
            .borrow_mut()
            .registered_gateways
            .read(&registered_gateway_index)
            .is_ok()
    })
}

#[update]
#[candid_method(update)]
async fn init_gateway_by_ip(
    nonce: IpChallengeNonce,
    gateway_principal_id: GatewayPrincipalId,
) -> GenericResult<GatewayPrincipalId> {
    caller_is_omnia_backend();

    STATE.with(|state| {
        // validate IP challenge
        let ip_challenge_value = state
            .borrow_mut()
            .ip_challenges
            .validate_ip_challenge_by_nonce(nonce)?;

        // create initialized gateway, if not already initialized
        let initialized_gateway_index = InitializedGatewayIndex {
            ip: ip_challenge_value.requester_ip,
        };

        if !state
            .borrow()
            .initialized_gateways
            .is_gateway_initialized(initialized_gateway_index.clone())
        {
            let initialized_gateway_value = InitializedGatewayValue {
                /// gateway principal ID
                principal_id: gateway_principal_id.clone(),
                /// UID of the proxied gateway (if any)
                // needed because when registering the gateway in environment, the request comes from the manager which is never proxied
                proxied_gateway_uid: ip_challenge_value.proxied_gateway_uid
            };
            state
                .borrow_mut()
                .initialized_gateways
                .create(initialized_gateway_index, initialized_gateway_value)
                .expect("previous entry should not exist");
            print(format!(
                "Initialized gateway with prinipal ID: {:?}",
                gateway_principal_id
            ));
        }
        Ok(gateway_principal_id)
    })
}

#[update]
#[candid_method(update)]
async fn get_initialized_gateways_by_ip(
    nonce: IpChallengeNonce,
) -> GenericResult<Vec<InitializedGatewayValue>> {
    caller_is_omnia_backend();

    STATE.with(|state| {
        // validate IP challenge
        let ip_challenge_value = state
            .borrow_mut()
            .ip_challenges
            .validate_ip_challenge_by_nonce(nonce)?;

        // get initialized gateways by IP
        let initialized_gateway_index = InitializedGatewayIndex {
            ip: ip_challenge_value.requester_ip,
        };
        match state
            .borrow_mut()
            .initialized_gateways
            .read(&initialized_gateway_index)
        {
            Ok(initialized_gateway_value) => Ok(vec![initialized_gateway_value.to_owned()]),
            Err(e) => Err(e),
        }
    })
}

#[update]
#[candid_method(update)]
async fn create_new_environment(
    environment_manager_principal_id: VirtualPersonaPrincipalId,
    environment_creation_input: EnvironmentCreationInput,
) -> GenericResult<EnvironmentCreationResult> {
    caller_is_omnia_backend();

    let environment_uid = Uuid::new_v4().hyphenated().to_string();

    STATE.with(|state| {
        // create new environment
        print(format!(
            "Creating new environment: {:?} managed by: {:?}",
            environment_creation_input, environment_manager_principal_id
        ));
        print(format!("New environment UID: {:?}", environment_uid));
        let environment_index = EnvironmentIndex {
            environment_uid: environment_uid.clone(),
        };
        let environment_value = EnvironmentValue {
            env_name: environment_creation_input.env_name.clone(),
            env_ip: None,
            env_users_principals_ids: BTreeMap::default(),
            env_gateways_principals_ids: BTreeMap::default(),
            env_manager_principal_id: environment_manager_principal_id.clone(),
        };
        state
            .borrow_mut()
            .environments
            .create(environment_index, environment_value)
            .expect("previous entry should not exist");

        // update manager environment in virtual persona
        let virtual_persona_index = VirtualPersonaIndex {
            principal_id: environment_manager_principal_id,
        };
        state
            .borrow_mut()
            .virtual_personas
            .insert_env_in_virtual_persona_as_manager(
                virtual_persona_index,
                environment_uid.clone(),
            )?;

        let environment_creation_result = EnvironmentCreationResult {
            env_name: environment_creation_input.env_name,
            env_uid: environment_uid,
        };
        print(format!(
            "Created new environment: {:?}",
            environment_creation_result
        ));
        Ok(environment_creation_result)
    })
}

#[update]
#[candid_method(update)]
fn register_gateway_in_environment(
    nonce: IpChallengeNonce,
    environment_manager_principal_id: VirtualPersonaPrincipalId,
    gateway_registration_input: GatewayRegistrationInput,
) -> RegisteredGatewayResult {
    caller_is_omnia_backend();

    STATE.with(|state| {
        // validate IP challenge
        let ip_challenge_value = state
            .borrow_mut()
            .ip_challenges
            .validate_ip_challenge_by_nonce(nonce)?;

        // remove initialized gateways
        // we only get the initialized gateway value if the registration request (from the managaer) comes from the same network of the initialized gateway
        let initialized_gateway_index = InitializedGatewayIndex {
            ip: ip_challenge_value.requester_ip.clone(), // manager's IP
        };
        let initialized_gateway_value = state
            .borrow_mut()
            .initialized_gateways
            .delete(&initialized_gateway_index)?;
        // register mapping IP to Environment UID in order to be able to retrive the UID of the environment from the IP when a User registers in an environment
        let environment_uid_index = EnvironmentUidIndex {
            ip: ip_challenge_value.requester_ip.clone(),
        };
        let environment_uid_value = EnvironmentUidValue {
            env_uid: gateway_registration_input.env_uid.clone(),
        };
        state
            .borrow_mut()
            .environment_uids
            .create(environment_uid_index, environment_uid_value)
            .expect("previous entry should not exist");

        // created registered gateway
        print(format!(
            "Registering gateway in environment with UID: {:?} managed by: {:?}",
            gateway_registration_input.env_uid, environment_manager_principal_id
        ));
        let registered_gateway_index = RegisteredGatewayIndex {
            principal_id: initialized_gateway_value.principal_id,
        };
        let registered_gateway_value = RegisteredGatewayValue {
            gateway_name: gateway_registration_input.gateway_name,
            gateway_ip: ip_challenge_value.requester_ip.clone(),
            gateway_url: get_gateway_url(
                ip_challenge_value.requester_ip,
                initialized_gateway_value.proxied_gateway_uid.is_some(), // true if gateway is proxied (determined during intialization)
            ),
            proxied_gateway_uid: initialized_gateway_value.proxied_gateway_uid,
            env_uid: gateway_registration_input.env_uid.clone(),
            gat_registered_device_uids: BTreeMap::default(),
        };
        state.borrow_mut().registered_gateways.create(
            registered_gateway_index.clone(),
            registered_gateway_value.clone(),
        )?;

        // add principal ID of registered Gateway to Environment
        let environment_index = EnvironmentIndex {
            environment_uid: gateway_registration_input.env_uid,
        };
        state
            .borrow_mut()
            .environments
            .insert_gateway_principal_id_in_env(
                environment_index,
                registered_gateway_index.principal_id,
            )?;

        Ok(registered_gateway_value)
    })
}

#[update]
#[candid_method(update)]
fn get_registered_gateways_in_environment(
    environment_uid: EnvironmentUID,
) -> MultipleRegisteredGatewayResult {
    caller_is_omnia_backend();

    STATE.with(|state| {
        // get principal IDs of gateways registered in environment
        let environment_index = EnvironmentIndex { environment_uid };
        let environment_value = state
            .borrow()
            .environments
            .read(&environment_index)?
            .clone();
        let gateway_principal_ids: Vec<GatewayPrincipalId> =
            environment_value.env_gateways_principals_ids.iter().fold(
                vec![],
                |mut gateway_principal_ids, (gateway_principal_id, _)| {
                    gateway_principal_ids.push(gateway_principal_id.clone());
                    gateway_principal_ids
                },
            );

        // get registered gateways by principal ID
        let mut registered_gateways: Vec<RegisteredGatewayValue> = vec![];
        for gateway_principal_id in gateway_principal_ids {
            let registered_gateway_index = RegisteredGatewayIndex {
                principal_id: gateway_principal_id,
            };
            let registered_gateway_value = state
                .borrow()
                .registered_gateways
                .read(&registered_gateway_index)?
                .clone();
            registered_gateways.push(registered_gateway_value.clone());
        }
        print(format!("Registered gateways: {:?}", registered_gateways));
        Ok(registered_gateways)
    })
}

#[update]
#[candid_method(update)]
fn get_gateway_updates_by_principal(gateway_principal_id: GatewayPrincipalId) -> UpdateValueOption {
    caller_is_omnia_backend();

    STATE.with(|state| {
        // get updates for gateway
        let update_index = UpdateIndex {
            gateway_principal_id,
        };

        // check if there are any update
        let is_update = state.borrow().updates.read(&update_index).is_ok();

        if is_update {
            let update_value = state
                .borrow_mut()
                .updates
                .delete(&update_index)
                .expect("must have update");
            return Some(update_value);
        }
        None
    })
}

#[update]
#[candid_method(update)]
fn pair_new_device_on_gateway(
    nonce: IpChallengeNonce,
    manager_principal_id: VirtualPersonaPrincipalId,
    gateway_principal_id: GatewayPrincipalId,
    pairing_payload: PairingPayload,
) -> UpdateValueResult {
    caller_is_omnia_backend();

    STATE.with(|state| {
        // validate IP challenge
        let ip_challenge_value = state
            .borrow_mut()
            .ip_challenges
            .validate_ip_challenge_by_nonce(nonce)?;

        // check if gateway is already registered
        let registered_gateway_index = RegisteredGatewayIndex {
            principal_id: gateway_principal_id.clone(),
        };
        let registered_gateway_value = state
            .borrow()
            .registered_gateways
            .read(&registered_gateway_index)?
            .clone();

        // check if pairing request is coming from the same network of the gateway
        if registered_gateway_value.gateway_ip == ip_challenge_value.requester_ip {
            // create updates for gateway
            let update_index = UpdateIndex {
                gateway_principal_id: gateway_principal_id.clone(),
            };

            let update_value = UpdateValue {
                virtual_persona_principal_id: manager_principal_id.clone(),
                virtual_persona_ip: ip_challenge_value.requester_ip,
                command: String::from("pair"),
                info: PairingInfo {
                    payload: pairing_payload,
                },
            };

            state
                .borrow_mut()
                .updates
                .create(update_index, update_value.clone())?;

            print(format!(
                "Manager {:?} paired new device to gateway {:?}",
                manager_principal_id, gateway_principal_id
            ));
            return Ok(update_value);
        }
        Err(String::from(
            "Cannot commission devices from a different network of the gateway",
        ))
    })
}

#[update]
#[candid_method(update)]
async fn register_device_on_gateway(
    nonce: IpChallengeNonce,
    gateway_principal_id: GatewayPrincipalId,
) -> RegisteredDeviceResult {
    caller_is_omnia_backend();

    let device_uid = Uuid::new_v4().hyphenated().to_string();

    STATE.with(|state| {
        // validate IP challenge
        let ip_challenge_value = state
            .borrow_mut()
            .ip_challenges
            .validate_ip_challenge_by_nonce(nonce)?;

        // check if gateway is already registered
        let registered_gateway_index = RegisteredGatewayIndex {
            principal_id: gateway_principal_id.clone(),
        };
        let registered_gateway_value = state
            .borrow()
            .registered_gateways
            .read(&registered_gateway_index)?
            .clone();

        // check if pairing request is coming from the same network of the gateway
        if registered_gateway_value.gateway_ip == ip_challenge_value.requester_ip {
            let registered_device_index = RegisteredDeviceIndex {
                device_uid: device_uid.clone(),
            };

            let registered_device_value = RegisteredDeviceValue {
                gateway_principal_id: gateway_principal_id.clone(),
                env_uid: registered_gateway_value.env_uid.clone(),
                device_url: get_device_url(
                    registered_gateway_value.gateway_url,
                    device_uid.clone(),
                ),
                required_headers: registered_gateway_value.proxied_gateway_uid.map(
                    |proxied_gateway_uid| {
                        vec![
                            (String::from("X-Forward-To-Peer"), proxied_gateway_uid),
                            // this is the port where the gateway is running the WoT servient
                            // TODO: store this value in the Gateway state, because it could be different for each gateway
                            // why 8080? That's because the WoT servient is behind an NGINX reverse proxy that handles idempotency for requests coming from the canisters
                            // see https://omnia-network/omnia-gateway README for details
                            (String::from("X-Forward-To-Port"), "8080".to_string()),
                        ]
                    },
                ),
            };

            // register device in gateway
            state
                .borrow_mut()
                .registered_gateways
                .insert_device_uid_in_gateway(registered_gateway_index, device_uid.clone())?;

            state.borrow_mut().registered_devices.create(
                registered_device_index.clone(),
                registered_device_value.clone(),
            )?;
            print(format!(
                "Gateway {:?} registered new device with UID {:?}",
                gateway_principal_id, device_uid
            ));

            return Ok((registered_device_index, registered_device_value));
        }
        Err(String::from(
            "Cannot register device from a different network of the gateway",
        ))
    })
}

#[update]
#[candid_method(update)]
async fn get_registered_devices_on_gateway(
    gateway_principal_id: GatewayPrincipalId,
) -> RegisteredDevicesUidsResult {
    caller_is_omnia_backend();

    STATE.with(|state| {
        // check if gateway is already registered
        let registered_gateway_index = RegisteredGatewayIndex {
            principal_id: gateway_principal_id.clone(),
        };
        let registered_gateway_value = state
            .borrow()
            .registered_gateways
            .read(&registered_gateway_index)?
            .clone();

        // return registered devices
        Ok(registered_gateway_value
            .gat_registered_device_uids
            .keys()
            .cloned()
            .collect())
    })
}
