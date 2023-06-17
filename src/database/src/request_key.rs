use candid::candid_method;
use ic_cdk::print;
use ic_cdk_macros::update;
use omnia_types::{
    errors::GenericResult,
    request_key::{RequestKeyCreationResult, RequestKeyIndex, RequestKeyValue, UniqueRequestKey},
};
use uuid::Uuid;

use crate::{utils::caller_is_omnia_backend, STATE};

#[update(name = "createNewRequestKey")]
#[candid_method(update, rename = "createNewRequestKey")]
fn create_new_request_key(owner: String) -> RequestKeyCreationResult {
    caller_is_omnia_backend();

    let request_key_uid = Uuid::new_v4().hyphenated().to_string();

    STATE.with(|state| {
        // create new request key
        print(format!(
            "Creating new request key: {:?} owned by: {:?}",
            request_key_uid, owner
        ));

        let request_key_index = RequestKeyIndex {
            request_key_uid: request_key_uid.clone(),
        };

        let request_key_value = RequestKeyValue::new(request_key_uid, owner);

        state
            .borrow_mut()
            .valid_request_keys
            .create(request_key_index, request_key_value.clone())?;

        Ok(request_key_value)
    })
}

#[update(name = "spendRequestForKey")]
#[candid_method(update, rename = "spendRequestForKey")]
fn spend_request_for_key(unique_request_key: UniqueRequestKey) -> GenericResult<RequestKeyValue> {
    caller_is_omnia_backend();

    STATE.with(|state| {
        let request_key_index = RequestKeyIndex {
            request_key_uid: unique_request_key.get_uid(),
        };

        let request_key_value = state
            .borrow()
            .valid_request_keys
            .read(&request_key_index)?
            .clone();

        if request_key_value.is_used_nonce(unique_request_key.get_nonce()) {
            // TODO: disqualify request key
            return Err(String::from("Nonce has already been used"));
        }

        let updated_request_key_value = request_key_value.increment_counter();

        state
            .borrow_mut()
            .valid_request_keys
            .update(request_key_index, updated_request_key_value.clone())?;

        Ok(updated_request_key_value)
    })
}
