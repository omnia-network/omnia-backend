use candid::candid_method;
use ic_cdk::print;
use ic_cdk_macros::update;
use omnia_types::{
    environment::{EnvironmentCreationInput, EnvironmentCreationResult, EnvironmentUID, Environment},
    gateway::{
        RegisteredGateway, RegisteredGatewayResult, GatewayRegistrationInput,
        MultipleRegisteredGatewayResult, GatewayPrincipalId,
    },
    virtual_persona::VirtualPersonaPrincipalId, http::{CanisterCallNonce, RequesterInfo},
};

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

#[update(name = "getRegisteredGatewaysInEnvironment")]
#[candid_method(update, rename = "getRegisteredGatewaysInEnvironment")]
fn get_registered_gateways_in_environment(environment_uid: EnvironmentUID) -> MultipleRegisteredGatewayResult {
    let gateways_principal_ids_in_environment_result = STATE.with(
        |state| match state.borrow().environments.get(&environment_uid) {
            Some(environment) => Ok(environment.env_gateway_principal_ids.clone()),
            None => {
                let err = format!("Environmnent: {:?} does not exist", environment_uid);

                print(err.as_str());
                Err(err)
            }
        },
    );

    match gateways_principal_ids_in_environment_result {
        Ok(gateways_principal_ids_in_environment) => {
            let mut registered_gateways: Vec<RegisteredGateway> = vec![];
            for gateway_principal_id in gateways_principal_ids_in_environment {
                STATE.with(|state| {
                    match state
                        .borrow()
                        .registered_gateways
                        .get(&gateway_principal_id) 
                    {
                        Some(registered_gateway) => registered_gateways.push(registered_gateway.clone()),
                        None => ()
                    };
                });
            }
            Ok(registered_gateways)
        }
        Err(e) => Err(e) 
    }
}
