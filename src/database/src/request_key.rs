use candid::candid_method;
use ic_cdk::print;
use ic_cdk_macros::update;
use omnia_types::request_key::{RequestKeyCreationResult, RequestKeyIndex, RequestKeyValue};
use uuid::Uuid;

use crate::{utils::caller_is_omnia_backend, STATE};

#[update(name = "createNewRequestKey")]
#[candid_method(update, rename = "createNewRequestKey")]
fn create_new_request_key(owner: String) -> RequestKeyCreationResult {
    caller_is_omnia_backend();

    let request_key_uid = Uuid::new_v4().hyphenated().to_string();

    STATE.with(|state| {
        // create new environment
        print(format!(
            "Creating new request key: {:?} owned by: {:?}",
            request_key_uid, owner
        ));

        let request_key_index = RequestKeyIndex {
            request_key_uid: request_key_uid.clone(),
        };

        let request_key_value = RequestKeyValue {
            key: request_key_uid,
            owner,
            counter: 0,
        };

        state
            .borrow_mut()
            .valid_request_keys
            .create(request_key_index, request_key_value.clone())?;

        Ok(request_key_value)
    })
}
