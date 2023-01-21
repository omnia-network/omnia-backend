use candid::CandidType;
use candid::{candid_method, Deserialize};
use ic_cdk::{export::Principal, print};
use ic_cdk_macros::update;
use omnia_types::{
    environment::{EnvironmentInfo, EnvironmentUID},
    user::PrincipalId,
};
use serde::Serialize;

use crate::STATE;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct StoredUserProfile {
    pub user_principal_id: PrincipalId,
    pub environment_uid: Option<EnvironmentUID>,
}

#[update(name = "setUserInEnvironment")]
#[candid_method(update, rename = "setUserInEnvironment")]
fn set_user_in_environment(
    user_principal_id: PrincipalId,
    env_uid: EnvironmentUID,
) -> EnvironmentInfo {
    let user_principal = Principal::from_text(user_principal_id).unwrap();

    match get_user_profile_if_exists(user_principal) {
        Some(user_profile) => {
            let (env_uid, env_name, env_manager_principal_id) =
                STATE.with(|state| match state.borrow().environments.get(&env_uid) {
                    Some(environment_info) => (
                        env_uid,
                        environment_info.env_name.clone(),
                        environment_info.env_manager_principal_id.clone(),
                    ),
                    None => panic!("Environment does not exist"),
                });

            let updated_user_profile = StoredUserProfile {
                environment_uid: Some(env_uid.to_owned()),
                ..user_profile
            };

            STATE.with(|state| {
                state
                    .borrow_mut()
                    .user_profiles
                    .insert(user_principal, updated_user_profile)
            });

            print(format!(
                "User: {:?} set in environment with UUID: {:?}",
                user_principal, env_uid
            ));

            EnvironmentInfo {
                env_name,
                env_uid,
                env_manager_principal_id,
            }
        }
        None => panic!("User does not have a profile"),
    }
}

#[update(name = "resetUserFromEnvironment")]
#[candid_method(update, rename = "resetUserFromEnvironment")]
fn reset_user_from_environment(user_principal_id: PrincipalId) -> EnvironmentInfo {
    let user_principal = Principal::from_text(user_principal_id).unwrap();

    match get_user_profile_if_exists(user_principal) {
        Some(user_profile) => {
            let updated_user_profile = StoredUserProfile {
                environment_uid: None,
                ..user_profile
            };

            STATE.with(|state| {
                state
                    .borrow_mut()
                    .user_profiles
                    .insert(user_principal, updated_user_profile)
            });

            match user_profile.environment_uid {
                Some(old_environment_uid) => STATE.with(|state| {
                    match state.borrow().environments.get(&old_environment_uid) {
                        Some(environment_info) => EnvironmentInfo {
                            env_name: environment_info.env_name.clone(),
                            env_uid: old_environment_uid,
                            env_manager_principal_id: environment_info
                                .env_manager_principal_id
                                .clone(),
                        },
                        None => panic!("Environment does not exist"),
                    }
                }),
                None => panic!("User is not in environment"),
            }
        }
        None => panic!("User does not have a profile"),
    }
}

#[update(name = "getUserProfile")]
#[candid_method(update, rename = "getUserProfile")]
fn get_user_profile(user_principal_id: PrincipalId) -> StoredUserProfile {
    let user_principal = Principal::from_text(user_principal_id).unwrap();

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
            let new_user_profile = StoredUserProfile {
                user_principal_id: user_principal.to_string(),
                environment_uid: None,
            };

            print(format!(
                "Created profile: {:?} of user: {:?}",
                new_user_profile, user_principal
            ));

            STATE.with(|state| {
                state
                    .borrow_mut()
                    .user_profiles
                    .insert(user_principal, new_user_profile.clone());
            });

            new_user_profile
        }
    }
}

fn get_user_profile_if_exists(user_principal: Principal) -> Option<StoredUserProfile> {
    STATE.with(
        |state| match state.borrow().user_profiles.get(&user_principal) {
            Some(user_profile) => Some(user_profile.to_owned()),
            None => None,
        },
    )
}
