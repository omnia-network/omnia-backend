use candid::candid_method;
use ic_cdk::api::management_canister::provisional::CanisterId;
use ic_cdk_macros::update;
use omnia_core_sdk::{
    access_key::generate_signed_unique_access_key, init_client, ledger::request_access_key,
};
use omnia_core_sdk::{access_key::AccessKeyUID, signature::SignatureReply};

use omnia_types::errors::GenericResult;
use omnia_utils::random::get_seeded_rng;

#[update]
#[candid_method(update)]
async fn get_access_key(
    ledger_canister_id: CanisterId,
    omnia_canister_id: CanisterId,
) -> GenericResult<AccessKeyUID> {
    init_client(ledger_canister_id, omnia_canister_id, get_seeded_rng());
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
