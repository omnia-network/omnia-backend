use std::{collections::BTreeMap, cell::RefMut};
use candid::candid_method;
use ic_cdk::print;
use ic_cdk_macros::update;
use omnia_types::{
    environment::{EnvironmentCreationInput, EnvironmentCreationResult, EnvironmentUID, Environment},
    gateway::{
        RegisteredGateway, RegisteredGatewayResult, GatewayRegistrationInput,
        MultipleRegisteredGatewayResult, GatewayPrincipalId,
    },
    virtual_persona::VirtualPersonaPrincipalId, http::CanisterCallNonce, errors::GenericError,
};

use crate::{uuid::generate_uuid, STATE, State};

#[update(name = "initGatewayWithIp")]
#[candid_method(update, rename = "initGatewayWithIp")]
async fn init_gateway_with_ip(nonce: CanisterCallNonce, gateway_principal_id: GatewayPrincipalId) -> Result<String, ()> {

    STATE.with(|state| {
        let mut mutable_state = state.borrow_mut();
        match mutable_state
            .initialized_nonce_to_ip
            .remove(&nonce)
        {
            Some(gateway_request_info) => {        
                mutable_state
                    .initialized_gateways
                    .insert(gateway_request_info.requester_ip, gateway_principal_id.clone());
                Ok(gateway_principal_id)
    
            },
            None => {
                Err(())
            }
        }
    })
}

#[update(name = "getInitializedGatewaysByIp")]
#[candid_method(update, rename = "getInitializedGatewaysByIp")]
async fn get_initialized_gateways_by_ip(nonce: CanisterCallNonce) -> Result<Vec<GatewayPrincipalId>, ()> {
    STATE.with(|state| {
        let mut mutable_state = state.borrow_mut();
        match mutable_state
            .initialized_nonce_to_ip
            .remove(&nonce)
        {
            Some(virtual_persona_request_info) => {
                match mutable_state
                    .initialized_gateways
                    .get(&virtual_persona_request_info.requester_ip) 
                {
                    Some(gateway_uid) => {
                        Ok(vec![gateway_uid.to_owned()])
                    },
                    None => Ok(vec![])
                }
            },
            None => {
                Err(())
            }
        }
    })
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
        let mut mutable_state = state.borrow_mut();
        mutable_state.environments.insert(
            environment_uid.clone(),
            Environment {
                env_name: environment_creation_input.env_name.clone(),
                env_ip: None,
                env_users_principals_ids: BTreeMap::default(),
                env_gateway_principal_ids: BTreeMap::default(),
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

    STATE.with(|state| {
        let mut mutable_state = state.borrow_mut();
        match mutable_state
            .initialized_nonce_to_ip
            .remove(&nonce)
        {
            Some(virtual_persona_request_info) => {
                match mutable_state
                    .initialized_gateways
                    .remove(&virtual_persona_request_info.requester_ip)
                {
                    Some(gateway_principal_id) => {
                        // register mapping IP to Environment UID in order to be able to retrive the UID of the environment from the IP when a User registers in an environment
                        mutable_state
                            .ip_to_env_uid
                            .insert(virtual_persona_request_info.requester_ip.clone(), gateway_registration_input.env_uid.clone());

                        let registered_gateway = RegisteredGateway {
                            gateway_name: gateway_registration_input.gateway_name,
                            gateway_ip: virtual_persona_request_info.requester_ip,
                            env_uid: gateway_registration_input.env_uid.clone(),
                        };

                        mutable_state
                            .registered_gateways
                            .insert(gateway_principal_id.clone(), registered_gateway.clone());

                        match mutable_state
                            .environments
                            .get_mut(&gateway_registration_input.env_uid)
                        {
                            Some(environment) => {
                                print(format!(
                                    "Registering gateway in environment with UID: {:?} managed by: {:?}",
                                    gateway_registration_input.env_uid,
                                    environment_manager_principal_id
                                ));

                                // add principal ID of registered Gateway to Environment
                                environment.env_gateway_principal_ids.insert(gateway_principal_id, ());
                                print(format!("Updated environment: {:?}", environment));
                                Ok(registered_gateway)
                            },
                            None => {
                                let err = format!(
                                    "Environment with uid {:?} does not exist",
                                    gateway_registration_input.env_uid
                                );
                    
                                print(err.as_str());
                                Err(err)
                            },
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
    })
}

#[update(name = "getRegisteredGatewaysInEnvironment")]
#[candid_method(update, rename = "getRegisteredGatewaysInEnvironment")]
fn get_registered_gateways_in_environment(environment_uid: EnvironmentUID) -> MultipleRegisteredGatewayResult {
    STATE.with(|state| {
        let mut mutable_state = state.borrow_mut();
        match get_environment_from_uid(&mut mutable_state, &environment_uid) {
            Ok(environment) => {
                print(format!("{:?}", environment));
                let mut registered_gateways: Vec<RegisteredGateway> = vec![];
                print(format!("{:?}", environment.env_gateway_principal_ids));
                for (gateway_principal_id, _) in environment.env_gateway_principal_ids {
                    print(format!("{:?}", gateway_principal_id));
                    match mutable_state
                        .registered_gateways
                        .get(&gateway_principal_id) 
                    {
                        Some(registered_gateway) => registered_gateways.push(registered_gateway.clone()),
                        None => ()
                    };
                }
                print(format!("{:?}", registered_gateways));
                Ok(registered_gateways)
            }
            Err(e) => Err(e) 
        }
    })

}

fn get_environment_from_uid(mutable_state: &mut RefMut<State>, env_uid: &EnvironmentUID) -> Result<Environment, GenericError> {
    match mutable_state
        .environments
        .get_mut(env_uid)
    {
        Some(environment) => Ok(environment.clone()),
        None => {
            let err = format!(
                "Environment with uid {:?} does not exist",
                env_uid
            );

            print(err.as_str());
            Err(err)
        },
    }
}