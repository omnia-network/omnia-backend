use candid::candid_method;
use ic_cdk::print;
use ic_cdk_macros::update;
use omnia_core_sdk::access_key::UniqueAccessKey;
use omnia_types::{
    access_key::{AccessKeyCreationArgs, AccessKeyCreationResult, AccessKeyIndex, AccessKeyValue},
    errors::GenericResult,
};
use uuid::Uuid;

use crate::{utils::caller_is_omnia_backend, STATE};

#[update]
#[candid_method(update)]
fn create_new_access_key(args: AccessKeyCreationArgs) -> AccessKeyCreationResult {
    caller_is_omnia_backend();

    STATE.with(|state| {
        print(format!("Requested new access key, args: {:?}", args));

        // check if there's alreay an access key with the same transaction hash
        if state
            .borrow()
            .valid_access_keys
            .transaction_hash_exists(args.transaction_hash)
        {
            return Err(String::from(
                "Access key with the same transaction hash already exists",
            ));
        }

        // create new access key
        // TODO: generate an access key that is not a UUIDv4
        let access_key_uid = Uuid::new_v4().simple().to_string();

        print(format!("Creating new access key: {:?}", access_key_uid));

        let access_key_index = AccessKeyIndex {
            access_key_uid: access_key_uid.clone(),
        };

        let access_key_value =
            AccessKeyValue::new(access_key_uid, args.owner, args.transaction_hash);

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
            access_key_uid: unique_access_key.get_key(),
        };

        let mut access_key_value = state
            .borrow()
            .valid_access_keys
            .read(&access_key_index)?
            .clone();

        let nonce = unique_access_key.get_nonce();

        if access_key_value.is_used_nonce(nonce) {
            // TODO: disqualify access key
            return Err(String::from("Nonce has already been used"));
        }

        access_key_value.spend_nonce(nonce);

        state
            .borrow_mut()
            .valid_access_keys
            .update(access_key_index, access_key_value.clone())?;

        Ok(access_key_value)
    })
}
