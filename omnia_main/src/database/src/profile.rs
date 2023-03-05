use candid::CandidType;
use candid::{candid_method, Deserialize};
use ic_cdk::{export::Principal, print, trap};
use ic_cdk_macros::update;
use omnia_types::environment::EnvironmentInfoResult;
use omnia_types::{
    environment::{EnvironmentInfo, EnvironmentUID},
    user::PrincipalId,
};
use omnia_utils::get_principal_from_string;
use serde::Serialize;

use crate::STATE;

type Ip = String;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct VirtualPersona {
    pub virtual_persona_principal_id: PrincipalId,
    pub virtual_persona_ip: Ip,
    pub user_env_uid: Option<EnvironmentUID>,
    pub manager_env_uid: Option<EnvironmentUID>
}

#[update(name = "setUserInEnvironment")]
#[candid_method(update, rename = "setUserInEnvironment")]
fn set_user_in_environment(
    user_principal_id: PrincipalId,
    env_uid: EnvironmentUID,
) -> EnvironmentInfoResult {
    let user_principal = get_principal_from_string(user_principal_id);

    match get_user_profile_if_exists(user_principal) {
        Some(user_profile) => {
            let (env_uid, env_name, env_manager_principal_id) =
                STATE.with(|state| match state.borrow().environments.get(&env_uid) {
                    Some(environment_info) => (
                        env_uid,
                        environment_info.env_name.clone(),
                        environment_info.env_manager_principal_id.clone(),
                    ),
                    None => trap("Environment does not exist"),
                });

            let updated_user_profile = VirtualPersona {
                user_env_uid: Some(env_uid.to_owned()),
                ..user_profile
            };

            STATE.with(|state| {
                state
                    .borrow_mut()
                    .virtual_personas
                    .insert(user_principal, updated_user_profile)
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

    match get_user_profile_if_exists(user_principal) {
        Some(user_profile) => {
            let updated_user_profile = VirtualPersona {
                user_env_uid: None,
                ..user_profile
            };

            STATE.with(|state| {
                state
                    .borrow_mut()
                    .virtual_personas
                    .insert(user_principal, updated_user_profile)
            });

            match user_profile.user_env_uid {
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

#[update(name = "getUserProfile")]
#[candid_method(update, rename = "getUserProfile")]
fn get_user_profile(user_principal_id: PrincipalId) -> VirtualPersona {
    let user_principal = get_principal_from_string(user_principal_id);

    match get_user_profile_if_exists(user_principal) {
        Some(user_profile) => {
            print(format!(
                "User: {:?} has profile: {:?}",
                user_principal, user_profile
            ));
            user_profile
        }
        None => {
            print("User does not have a profile");

            // create new user profile
            let new_user_profile = VirtualPersona {
                virtual_persona_principal_id: user_principal.to_string(),
                virtual_persona_ip: "".to_string(),
                user_env_uid: None,
                manager_env_uid: None,
            };

            STATE.with(|state| {
                state
                    .borrow_mut()
                    .virtual_personas
                    .insert(user_principal, new_user_profile.clone());
            });

            print(format!(
                "Created profile: {:?} of user: {:?}",
                new_user_profile, user_principal
            ));

            new_user_profile
        }
    }
}

fn get_user_profile_if_exists(user_principal: Principal) -> Option<VirtualPersona> {
    STATE.with(
        |state| match state.borrow().virtual_personas.get(&user_principal) {
            Some(user_profile) => Some(user_profile.to_owned()),
            None => None,
        },
    )
}
