use candid::candid_method;
use ic_cdk::{
    api::management_canister::{
        ecdsa::{sign_with_ecdsa, SignWithEcdsaArgument},
        provisional::CanisterId,
    },
    call,
};
use ic_cdk_macros::update;
use ic_ledger_types::{BlockIndex, Tokens};
use omnia_core_sdk::access_key::{AccessKeyUID, UniqueAccessKey};
use omnia_types::{
    errors::GenericResult,
    signature::{EcdsaKeyIds, SignatureReply},
};
use omnia_utils::{ic::transfer_to, random::generate_nonce};

#[update]
#[candid_method(update)]
async fn transfer_tokens_to_backend(
    ledger_canister_id: CanisterId,
    omnia_canister_id: CanisterId,
    amount: Tokens,
) -> GenericResult<BlockIndex> {
    transfer_to(ledger_canister_id, omnia_canister_id, amount).await
}

#[update]
#[candid_method(update)]
async fn obtain_access_key(
    omnia_canister_id: CanisterId,
    block_index: BlockIndex,
) -> GenericResult<AccessKeyUID> {
    call::<(BlockIndex,), (GenericResult<AccessKeyUID>,)>(
        omnia_canister_id,
        "obtainAccessKey",
        (block_index,),
    )
    .await
    .unwrap()
    .0
}

#[update]
#[candid_method(update)]
async fn sign_access_key(access_key: AccessKeyUID) -> GenericResult<SignatureReply> {
    let unique_access_key = UniqueAccessKey::new(generate_nonce(), access_key.clone());

    let request = SignWithEcdsaArgument {
        message_hash: unique_access_key.generate_hash().to_vec(),
        derivation_path: vec![],
        key_id: EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id(),
    };

    let (response,) = sign_with_ecdsa(request)
        .await
        .map_err(|e| format!("sign_with_ecdsa failed {:?}", e))?;

    Ok(SignatureReply {
        signature_hex: hex::encode(response.signature),
        unique_access_key,
    })
}

#[cfg(test)]
mod tests {
    use candid::export_service;
    use ic_cdk::api::management_canister::provisional::CanisterId;
    use ic_ledger_types::{BlockIndex, Tokens};
    use std::env;

    use omnia_core_sdk::access_key::AccessKeyUID;
    use omnia_types::errors::*;
    use omnia_types::signature::*;

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
