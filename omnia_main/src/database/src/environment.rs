use std::{collections::BTreeMap, rc::Rc, cell::RefCell};
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
        let requester_info_to_be_checked = state
            .borrow_mut()
            .initialized_nonce_to_ip
            .remove(&nonce);

        print(format!("Requester info to be checked: {:?}", requester_info_to_be_checked));
    
        match requester_info_to_be_checked {
            Some(gateway_request_info) => {        
                state
                    .borrow_mut()
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
        let requester_info_to_be_checked = state
            .borrow_mut()
            .initialized_nonce_to_ip
            .remove(&nonce);

        print(format!("Requester info to be checked: {:?}", requester_info_to_be_checked));

        match requester_info_to_be_checked {
            Some(virtual_persona_request_info) => {
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
        state.borrow_mut().environments.insert(
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
        let requester_info_to_be_checked = state
            .borrow_mut()
            .initialized_nonce_to_ip
            .remove(&nonce);
        match requester_info_to_be_checked {
            Some(virtual_persona_request_info) => {
                let gateway_principal_id_option = state
                    .borrow_mut()
                    .initialized_gateways
                    .remove(&virtual_persona_request_info.requester_ip);
                match gateway_principal_id_option {
                    Some(gateway_principal_id) => {
                        // register mapping IP to Environment UID in order to be able to retrive the UID of the environment from the IP when a User registers in an environment
                        state
                            .borrow_mut()
                            .ip_to_env_uid
                            .insert(virtual_persona_request_info.requester_ip.clone(), gateway_registration_input.env_uid.clone());

                        let registered_gateway = RegisteredGateway {
                            gateway_name: gateway_registration_input.gateway_name,
                            gateway_ip: virtual_persona_request_info.requester_ip,
                            env_uid: gateway_registration_input.env_uid.clone(),
                        };

                        state.borrow_mut()
                            .registered_gateways
                            .insert(gateway_principal_id.clone(), registered_gateway.clone());

                        match state
                            .borrow_mut()
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
        let environment_result = get_environment_from_uid(Rc::clone(&state), &environment_uid);

        match environment_result {
            Ok(environment) => {
                print(format!("{:?}", environment));
                let mut registered_gateways: Vec<RegisteredGateway> = vec![];
                print(format!("{:?}", environment.env_gateway_principal_ids));
                for (gateway_principal_id, _) in environment.env_gateway_principal_ids {
                    print(format!("{:?}", gateway_principal_id));
                    match state
                        .borrow()
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

fn get_environment_from_uid(state: Rc<RefCell<State>>, env_uid: &EnvironmentUID) -> Result<Environment, GenericError> {
    match state
        .borrow_mut()
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