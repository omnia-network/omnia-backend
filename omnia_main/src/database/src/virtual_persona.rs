use std::cell::RefMut;

use candid::candid_method;
use ic_cdk::{export::Principal, print, trap};
use ic_cdk_macros::{update, query};
use omnia_types::environment::{EnvironmentInfoResult, EnvironmentUID};
use omnia_types::http::CanisterCallNonce;
use omnia_types::{
    environment::EnvironmentInfo,
    virtual_persona::{VirtualPersona, VirtualPersonaPrincipalId},
};
use omnia_utils::get_principal_from_string;

use crate::{STATE, State};

#[update(name = "setUserInEnvironment")]
#[candid_method(update, rename = "setUserInEnvironment")]
fn set_user_in_environment(
    virtual_persona_principal_id: VirtualPersonaPrincipalId,
    nonce: CanisterCallNonce,
) -> EnvironmentInfoResult {
    let virtual_persona_principal = get_principal_from_string(virtual_persona_principal_id.clone());

    STATE.with(|state| {
        let mut mutable_state = state.borrow_mut();
        match mutable_state
            .initialized_nonce_to_ip
            .remove(&nonce)
        {
            Some(virtual_persona_request_info) => {
                match get_virtual_persona_if_exists(&mut mutable_state, virtual_persona_principal) {
                    Some(virtual_persona) => {
                        match get_environment_uid_from_ip(&mut mutable_state, &virtual_persona_request_info.requester_ip) {
                            Some(environment_uid) => {
                                match mutable_state
                                    .environments
                                    .get_mut(&environment_uid)
                                {
                                    Some(environment) => {
                                        environment.env_users_principals_ids.insert(virtual_persona_principal_id.clone(), ());
                                    },
                                    None => trap("Environment does not exist"),
                                };
                
                                let updated_virtual_persona = VirtualPersona {
                                    user_env_uid: Some(environment_uid.to_owned()),
                                    ..virtual_persona
                                };

                                mutable_state
                                    .virtual_personas
                                    .insert(virtual_persona_principal, updated_virtual_persona);

                                print(format!(
                                    "User: {:?} set in environment with UUID: {:?}",
                                    virtual_persona_principal_id, environment_uid
                                ));
                    
                                Ok(EnvironmentInfo {
                                    env_uid: environment_uid.clone(),
                                })
                            },
                            None => {
                                let err = format!(
                                    "No environment with IP: {}",
                                    virtual_persona_request_info.requester_ip
                                );
            
                                print(err.as_str());
                                Err(err)
                            }
                        }
                    }
                    None => {
                        let err = format!("User does not have a profile");
            
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
            }
        }
    })
}

#[update(name = "resetUserFromEnvironment")]
#[candid_method(update, rename = "resetUserFromEnvironment")]
fn reset_user_from_environment(virtual_persona_principal_id: VirtualPersonaPrincipalId, nonce: CanisterCallNonce) -> EnvironmentInfoResult {
    let virtual_persona_principal = get_principal_from_string(virtual_persona_principal_id.clone());
    STATE.with(|state| {
        let mut mutable_state = state.borrow_mut();
        match mutable_state
            .initialized_nonce_to_ip
            .remove(&nonce)
        {
            Some(virtual_persona_request_info) => {
                match get_virtual_persona_if_exists(&mut mutable_state, virtual_persona_principal) {
                    Some(virtual_persona) => {
                        let environment_uid_option = get_environment_uid_from_ip(&mut mutable_state, &virtual_persona_request_info.requester_ip);
                        match environment_uid_option {
                            Some(environment_uid) => {
                                match mutable_state
                                    .environments
                                    .get_mut(&environment_uid)
                                {
                                    Some(environment) => {
                                        environment.env_users_principals_ids.remove(&virtual_persona_principal_id);
                                    },
                                    None => trap("Environment does not exist"),
                                };

                                let updated_virtual_persona = VirtualPersona {
                                    user_env_uid: None,
                                    ..virtual_persona
                                };

                                mutable_state
                                    .virtual_personas
                                    .insert(virtual_persona_principal, updated_virtual_persona);

                                print(format!(
                                    "User: {:?} removed from environment with UUID: {:?}",
                                    virtual_persona_principal_id, environment_uid
                                ));

                                Ok(EnvironmentInfo {
                                    env_uid: environment_uid.clone(),
                                })
                            },
                            None => {
                                let err = format!(
                                    "No environment with IP: {}",
                                    virtual_persona_request_info.requester_ip
                                );

                                print(err.as_str());
                                Err(err)
                            }
                        }
                    }
                    None => {
                        let err = format!("User does not have a profile");

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

#[update(name = "getVirtualPersona")]
#[candid_method(update, rename = "getVirtualPersona")]
fn get_virtual_persona(nonce: CanisterCallNonce, virtual_persona_principal_id: VirtualPersonaPrincipalId) -> Result<VirtualPersona, ()> {
    STATE.with(|state| {
        let mut mutable_state = state.borrow_mut();
        match mutable_state
            .initialized_nonce_to_ip
            .remove(&nonce)
        {
            Some(virtual_persona_request_info) => {
                let virtual_persona_principal = get_principal_from_string(virtual_persona_principal_id);

                let virtual_persona_option = get_virtual_persona_if_exists(&mut mutable_state, virtual_persona_principal);

                match virtual_persona_option {
                    Some(virtual_persona) => {
                        print(format!(
                            "User: {:?} has profile: {:?}",
                            virtual_persona_principal, virtual_persona
                        ));
                        Ok(virtual_persona)
                    }
                    None => {
                        print("User does not have a profile");
    
                        // create new user profile
                        let new_virtual_persona = VirtualPersona {
                            virtual_persona_principal_id: virtual_persona_principal.to_string(),
                            virtual_persona_ip: virtual_persona_request_info.requester_ip,
                            user_env_uid: None,
                            manager_env_uid: None,
                        };

                        mutable_state
                            .create_virtual_persona(virtual_persona_principal, new_virtual_persona.clone());

                        print(format!(
                            "Created profile: {:?} of user: {:?}",
                            new_virtual_persona, virtual_persona_principal
                        ));

                        Ok(new_virtual_persona)
                    }
                }
            },
            None => {
                Err(())
            }
        }
    })
}

#[query(name = "checkIfVirtualPersonaExists")]
#[candid_method(query, rename = "checkIfVirtualPersonaExists")]
fn check_if_virtual_persona_exists(virtual_persona_principal: Principal) -> bool {
    STATE.with(
        |state| match state.borrow().virtual_personas.get(&virtual_persona_principal) {
            Some(_) => true,
            None => false,
        },
    )
}

fn get_virtual_persona_if_exists(mutable_state: &mut RefMut<State>, virtual_persona_principal: Principal) -> Option<VirtualPersona> {
    match mutable_state.virtual_personas.get(&virtual_persona_principal) {
        Some(virtual_persona) => Some(virtual_persona.to_owned()),
        None => None,
    }
}

fn get_environment_uid_from_ip(mutable_state: &mut RefMut<State>, requester_ip: &VirtualPersonaPrincipalId) -> Option<EnvironmentUID>{
    match mutable_state
        .ip_to_env_uid
        .get(requester_ip)
    {
        Some(env_uid) => Some(env_uid.clone()),
        None => None
    }
}