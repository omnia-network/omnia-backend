use candid::candid_method;
use ic_cdk::{export::Principal, print, trap};
use ic_cdk_macros::update;
use omnia_types::environment::EnvironmentInfoResult;
use omnia_types::http::{CanisterCallNonce, RequesterInfo};
use omnia_types::{
    environment::{EnvironmentInfo, EnvironmentUID},
    user::{VirtualPersona, PrincipalId},
};
use omnia_utils::get_principal_from_string;

use crate::STATE;

#[update(name = "setUserInEnvironment")]
#[candid_method(update, rename = "setUserInEnvironment")]
fn set_user_in_environment(
    user_principal_id: PrincipalId,
    env_uid: EnvironmentUID,
) -> EnvironmentInfoResult {
    let user_principal = get_principal_from_string(user_principal_id);

    match get_virtual_persona_if_exists(user_principal) {
        Some(virtual_persona) => {
            let (env_uid, env_name, env_manager_principal_id) =
                STATE.with(|state| match state.borrow().environments.get(&env_uid) {
                    Some(environment_info) => (
                        env_uid,
                        environment_info.env_name.clone(),
                        environment_info.env_manager_principal_id.clone(),
                    ),
                    None => trap("Environment does not exist"),
                });

            let updated_virtual_persona = VirtualPersona {
                user_env_uid: Some(env_uid.to_owned()),
                ..virtual_persona
            };

            STATE.with(|state| {
                state
                    .borrow_mut()
                    .virtual_personas
                    .insert(user_principal, updated_virtual_persona)
            });

            print(format!(
                "User: {:?} set in environment with UUID: {:?}",
                user_principal, env_uid
            ));

            Ok(EnvironmentInfo {
                env_name,
                env_uid,
                env_manager_principal_id,
            })
        }
        None => {
            let err = format!("User does not have a profile");

            print(err.as_str());

            Err(err)
        }
    }
}

#[update(name = "resetUserFromEnvironment")]
#[candid_method(update, rename = "resetUserFromEnvironment")]
fn reset_user_from_environment(user_principal_id: PrincipalId) -> EnvironmentInfoResult {
    let user_principal = get_principal_from_string(user_principal_id);

    match get_virtual_persona_if_exists(user_principal) {
        Some(virtual_persona) => {
            let updated_virtual_persona = VirtualPersona {
                user_env_uid: None,
                ..virtual_persona
            };

            STATE.with(|state| {
                state
                    .borrow_mut()
                    .virtual_personas
                    .insert(user_principal, updated_virtual_persona)
            });

            match virtual_persona.user_env_uid {
                Some(old_user_env_uid) => STATE.with(|state| {
                    match state.borrow().environments.get(&old_user_env_uid) {
                        Some(environment_info) => Ok(EnvironmentInfo {
                            env_name: environment_info.env_name.clone(),
                            env_uid: old_user_env_uid,
                            env_manager_principal_id: environment_info
                                .env_manager_principal_id
                                .clone(),
                        }),
                        None => {
                            let err = format!("Environment does not exist");

                            print(err.as_str());

                            Err(err)
                        }
                    }
                }),
                None => {
                    let err = format!("User is not in environment");

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
}

#[update(name = "getVirtualPersona")]
#[candid_method(update, rename = "getVirtualPersona")]
fn get_virtual_persona(nonce: CanisterCallNonce, user_principal_id: PrincipalId) -> Result<VirtualPersona, ()> {
    let requester_info_to_be_checked: Option<RequesterInfo> = STATE.with(|state| {
        state
            .borrow_mut()
            .initialized_nonce_to_ip
            .remove(&nonce)
    });

    print(format!("{:?}", requester_info_to_be_checked));

    match requester_info_to_be_checked {
        Some(virtual_persona_request_info) => {
            let user_principal = get_principal_from_string(user_principal_id);

            match get_virtual_persona_if_exists(user_principal) {
                Some(virtual_persona) => {
                    print(format!(
                        "User: {:?} has profile: {:?}",
                        user_principal, virtual_persona
                    ));
                    Ok(virtual_persona)
                }
                None => {
                    print("User does not have a profile");

                    // create new user profile
                    let new_virtual_persona = VirtualPersona {
                        virtual_persona_principal_id: user_principal.to_string(),
                        virtual_persona_ip: virtual_persona_request_info.requester_ip,
                        user_env_uid: None,
                        manager_env_uid: None,
                    };

                    STATE.with(|state| {
                        state
                            .borrow_mut()
                            .virtual_personas
                            .insert(user_principal, new_virtual_persona.clone());
                    });

                    print(format!(
                        "Created profile: {:?} of user: {:?}",
                        new_virtual_persona, user_principal
                    ));

                    Ok(new_virtual_persona)
                }
            }
        },
        None => {
            Err(())
        }
    }
}

fn get_virtual_persona_if_exists(user_principal: Principal) -> Option<VirtualPersona> {
    STATE.with(
        |state| match state.borrow().virtual_personas.get(&user_principal) {
            Some(virtual_persona) => Some(virtual_persona.to_owned()),
            None => None,
        },
    )
}
