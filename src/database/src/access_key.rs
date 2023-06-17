use candid::candid_method;
use ic_cdk::print;
use ic_cdk_macros::update;
use omnia_types::{
    access_key::{AccessKeyCreationResult, AccessKeyIndex, AccessKeyValue, UniqueAccessKey},
    errors::GenericResult,
};
use uuid::Uuid;

use crate::{utils::caller_is_omnia_backend, STATE};

#[update]
#[candid_method(update)]
fn create_new_access_key(owner: String) -> AccessKeyCreationResult {
    caller_is_omnia_backend();

    let access_key_uid = Uuid::new_v4().to_string();

    STATE.with(|state| {
        // create new request key
        print(format!(
            "Creating new request key: {:?} owned by: {:?}",
            access_key_uid, owner
        ));

        let access_key_index = AccessKeyIndex {
            access_key_uid: access_key_uid.clone(),
        };

        let access_key_value = AccessKeyValue::new(access_key_uid, owner);

        state
            .borrow_mut()
            .valid_access_keys
            .create(access_key_index, access_key_value.clone())?;

        Ok(access_key_value)
    })
}

#[update]
#[candid_method(update)]
fn spend_request_for_key(unique_access_key: UniqueAccessKey) -> GenericResult<AccessKeyValue> {
    caller_is_omnia_backend();

    STATE.with(|state| {
        let access_key_index = AccessKeyIndex {
            access_key_uid: unique_access_key.get_uid(),
        };

        let access_key_value = state
            .borrow()
            .valid_access_keys
            .read(&access_key_index)?
            .clone();

        if access_key_value.is_used_nonce(unique_access_key.get_nonce()) {
            // TODO: disqualify request key
            return Err(String::from("Nonce has already been used"));
        }

        let updated_access_key_value = access_key_value.increment_counter();

        state
            .borrow_mut()
            .valid_access_keys
            .update(access_key_index, updated_access_key_value.clone())?;

        Ok(updated_access_key_value)
    })
}
