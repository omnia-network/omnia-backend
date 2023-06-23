use candid::candid_method;
use ic_cdk::{api::management_canister::provisional::CanisterId};
use ic_cdk_macros::update;
use omnia_core_sdk::InitParams;
use omnia_core_sdk::{access_key::AccessKeyUID, signature::SignatureReply};
use omnia_core_sdk::{
    access_key::{generate_signed_unique_access_key, request_access_key},
    init_client,
};

use omnia_types::errors::GenericResult;

#[update]
#[candid_method(update)]
async fn get_access_key(
    omnia_canister_id: CanisterId,
    ledger_canister_id: CanisterId,
) -> GenericResult<AccessKeyUID> {
    // For simplicity, initialize the SDK client here.
    // It should be initialized in the init method of the canister.
    init_client(InitParams {
        omnia_canister_id: Some(omnia_canister_id),
        ledger_canister_id: Some(ledger_canister_id),
    });
    request_access_key().await
}

#[update]
#[candid_method(update)]
async fn sign_access_key(access_key: AccessKeyUID) -> GenericResult<SignatureReply> {
    generate_signed_unique_access_key(access_key).await
}

#[cfg(test)]
mod tests {
    use candid::export_service;
    use ic_cdk::api::management_canister::provisional::CanisterId;
    use std::env;

    use omnia_core_sdk::{access_key::AccessKeyUID, signature::SignatureReply};
    use omnia_types::errors::*;

    #[test]
    fn generate_candid_interface() {
        use std::fs::write;
        let dir = env::current_dir().unwrap();
        let did_name = "application_placeholder.did";
        let did_path = dir.join(did_name);

        export_service!();
        write(did_path, __export_service()).expect("Write failed.");
    }
}
