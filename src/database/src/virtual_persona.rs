use candid::candid_method;
use ic_cdk::print;
use ic_cdk_macros::{query, update};
use omnia_types::environment::{EnvironmentIndex, EnvironmentInfoResult, EnvironmentUidIndex};
use omnia_types::http::IpChallengeNonce;
use omnia_types::virtual_persona::{VirtualPersonaIndex, VirtualPersonaValueResult};
use omnia_types::{
    environment::EnvironmentInfo,
    virtual_persona::{VirtualPersonaPrincipalId, VirtualPersonaValue},
};

use crate::utils::caller_is_omnia_backend;
use crate::STATE;

#[update(name = "setUserInEnvironment")]
#[candid_method(update, rename = "setUserInEnvironment")]
fn set_user_in_environment(
    virtual_persona_principal_id: VirtualPersonaPrincipalId,
    nonce: IpChallengeNonce,
) -> EnvironmentInfoResult {
    caller_is_omnia_backend();

    STATE.with(|state| {
        // validate IP challenge
        let ip_challenge_value = state
            .borrow_mut()
            .ip_challenges
            .validate_ip_challenge_by_nonce(nonce)?;

        // update users in environment
        let environment_uid_index = EnvironmentUidIndex {
            ip: ip_challenge_value.requester_ip,
        };
        let environment_uid = state
            .borrow()
            .environment_uids
            .get_environment_uid_by_ip(environment_uid_index)?;
        let environment_index = EnvironmentIndex {
            environment_uid: environment_uid.clone(),
        };
        state
            .borrow_mut()
            .environments
            .insert_user_principal_id_in_env(
                environment_index,
                virtual_persona_principal_id.clone(),
            )?;

        // update user environment in virtual persona
        let virtual_persona_index = VirtualPersonaIndex {
            principal_id: virtual_persona_principal_id.clone(),
        };
        state
            .borrow_mut()
            .virtual_personas
            .insert_env_in_virtual_persona_as_user(
                virtual_persona_index,
                environment_uid.clone(),
            )?;

        print(format!(
            "User: {:?} set in environment with UUID: {:?}",
            virtual_persona_principal_id, environment_uid
        ));

        Ok(EnvironmentInfo {
            env_uid: environment_uid,
        })
    })
}

#[update(name = "resetUserFromEnvironment")]
#[candid_method(update, rename = "resetUserFromEnvironment")]
fn reset_user_from_environment(
    virtual_persona_principal_id: VirtualPersonaPrincipalId,
    nonce: IpChallengeNonce,
) -> EnvironmentInfoResult {
    caller_is_omnia_backend();

    STATE.with(|state| {
        // validate IP challenge
        let ip_challenge_value = state
            .borrow_mut()
            .ip_challenges
            .validate_ip_challenge_by_nonce(nonce)?;

        // update users in environment
        let environment_uid_index = EnvironmentUidIndex {
            ip: ip_challenge_value.requester_ip,
        };
        let environment_uid = state
            .borrow()
            .environment_uids
            .get_environment_uid_by_ip(environment_uid_index)?;
        let environment_index = EnvironmentIndex {
            environment_uid: environment_uid.clone(),
        };
        state
            .borrow_mut()
            .environments
            .remove_user_principal_id_in_env(
                environment_index,
                virtual_persona_principal_id.clone(),
            )?;

        // update user environment in virtual persona
        let virtual_persona_index = VirtualPersonaIndex {
            principal_id: virtual_persona_principal_id.clone(),
        };
        state
            .borrow_mut()
            .virtual_personas
            .remove_env_in_virtual_persona_as_user(virtual_persona_index)?;

        print(format!(
            "User: {:?} removed from environment with UUID: {:?}",
            virtual_persona_principal_id, environment_uid
        ));

        Ok(EnvironmentInfo {
            env_uid: environment_uid,
        })
    })
}

#[update(name = "getVirtualPersona")]
#[candid_method(update, rename = "getVirtualPersona")]
fn get_virtual_persona(
    nonce: IpChallengeNonce,
    virtual_persona_principal_id: VirtualPersonaPrincipalId,
) -> VirtualPersonaValueResult {
    caller_is_omnia_backend();

    STATE.with(|state| {
        // validate IP challenge
        let ip_challenge_value = state
            .borrow_mut()
            .ip_challenges
            .validate_ip_challenge_by_nonce(nonce)?;

        // if virtual persona exists, return it
        let virtual_persona_index = VirtualPersonaIndex {
            principal_id: virtual_persona_principal_id.clone(),
        };
        if let Ok(existing_virtual_persona_value) =
            state.borrow().virtual_personas.read(&virtual_persona_index)
        {
            print(format!(
                "User: {:?} has profile: {:?}",
                virtual_persona_index.principal_id, existing_virtual_persona_value
            ));
            return Ok(existing_virtual_persona_value.to_owned());
        }

        // otherwise, create a new one
        let new_virtual_persona_value = VirtualPersonaValue {
            virtual_persona_principal_id,
            virtual_persona_ip: ip_challenge_value.requester_ip,
            user_env_uid: None,
            manager_env_uid: None,
        };

        print(format!(
            "Created profile: {:?} of user: {:?}",
            new_virtual_persona_value, virtual_persona_index.principal_id
        ));

        state
            .borrow_mut()
            .virtual_personas
            .create(virtual_persona_index, new_virtual_persona_value.clone())
            .expect("previous entry should not exist");

        Ok(new_virtual_persona_value)
    })
}

#[query(name = "checkIfVirtualPersonaExists")]
#[candid_method(query, rename = "checkIfVirtualPersonaExists")]
fn check_if_virtual_persona_exists(
    virtual_persona_principal_id: VirtualPersonaPrincipalId,
) -> bool {
    caller_is_omnia_backend();

    let virtual_persona_index = VirtualPersonaIndex {
        principal_id: virtual_persona_principal_id,
    };
    STATE.with(|state| {
        state
            .borrow_mut()
            .virtual_personas
            .read(&virtual_persona_index)
            .is_ok()
    })
}
