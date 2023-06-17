use candid::Principal;
use ic_cdk::{
    api::{
        management_canister::{
            ecdsa::{ecdsa_public_key, EcdsaPublicKeyArgument, EcdsaPublicKeyResponse},
            provisional::CanisterId,
        },
        time,
    },
    print,
};
use ic_ledger_types::{
    account_balance, query_archived_blocks, query_blocks, transfer, AccountBalanceArgs,
    AccountIdentifier, Block, BlockIndex, GetBlocksArgs, Memo, Timestamp, Tokens, TransferArgs,
    DEFAULT_FEE, DEFAULT_SUBACCOUNT,
};
use k256::ecdsa::signature::Verifier;
use omnia_types::{errors::GenericResult, signature::EcdsaKeyIds};

use crate::STATE;

pub fn get_backend_principal() -> Principal {
    STATE
        .with(|state| state.borrow().backend_principal)
        .expect("No Backend canister principal")
}

pub fn update_backend_principal(backend_principal_id: String) {
    print(format!(
        "Backend canister Principal ID: {:?}",
        backend_principal_id
    ));

    let remote_principal: Principal =
        Principal::from_text(backend_principal_id).expect("Invalid Backend canister principal id");
    STATE.with(|state| {
        state.borrow_mut().backend_principal = Some(remote_principal);
    });
}

pub fn get_database_principal() -> Principal {
    STATE
        .with(|state| state.borrow().database_principal)
        .expect("No Database canister principal")
}

pub fn update_database_principal(database_principal_id: String) {
    print(format!(
        "Database canister Principal ID: {:?}",
        database_principal_id
    ));

    let remote_principal: Principal = Principal::from_text(database_principal_id)
        .expect("Invalid Database canister principal id");
    STATE.with(|state| {
        state.borrow_mut().database_principal = Some(remote_principal);
    });
}

pub fn update_ledger_principal(ledger_canister_principal_id: String) {
    print(format!(
        "Ledger canister Principal ID: {:?}",
        ledger_canister_principal_id
    ));

    let remote_principal: Principal = Principal::from_text(ledger_canister_principal_id)
        .expect("Invalid Ledger canister principal id");
    STATE.with(|state| {
        state.borrow_mut().ledger_principal = Some(remote_principal);
    });
}

pub async fn check_balance(principal: Principal) -> Tokens {
    let ledger_principal = STATE
        .with(|state| state.borrow().ledger_principal)
        .expect("should have provided ledger principal id");
    let balance = account_balance(
        ledger_principal,
        AccountBalanceArgs {
            account: AccountIdentifier::new(&principal, &DEFAULT_SUBACCOUNT),
        },
    )
    .await
    .expect("call to ledger failed");

    print(format!(
        "Balance of principal ID: {:?} is: {:?}",
        principal.to_string(),
        balance
    ));

    balance
}

pub async fn transfer_to(principal: Principal, amount: Tokens) -> BlockIndex {
    let ledger_principal = STATE
        .with(|state| state.borrow().ledger_principal)
        .expect("should have provided ledger principal id");

    let block_index = transfer(
        ledger_principal,
        TransferArgs {
            memo: Memo(0),
            amount,
            fee: DEFAULT_FEE,
            from_subaccount: None,
            to: AccountIdentifier::new(&principal, &DEFAULT_SUBACCOUNT),
            created_at_time: Some(Timestamp {
                timestamp_nanos: time(),
            }),
        },
    )
    .await
    .expect("call to ledger failed")
    .expect("error while transfering funds");

    print(format!(
        "Created block with index: {:?}, transferred: {:?} to principal ID: {:?}",
        block_index,
        amount,
        principal.to_string()
    ));

    block_index
}

pub async fn query_one_block(block_index: BlockIndex) -> GenericResult<Option<Block>> {
    let ledger_principal = STATE
        .with(|state| state.borrow().ledger_principal)
        .expect("should have provided ledger principal id");

    let args = GetBlocksArgs {
        start: block_index,
        length: 1,
    };

    if let Ok(blocks_result) = query_blocks(ledger_principal, args.clone()).await {
        if !blocks_result.blocks.is_empty() {
            debug_assert_eq!(blocks_result.first_block_index, block_index);
            return Ok(blocks_result.blocks.into_iter().next());
        }

        if let Some(func) = blocks_result.archived_blocks.into_iter().find_map(|b| {
            (b.start <= block_index && (block_index - b.start) < b.length).then_some(b.callback)
        }) {
            if let Ok(Ok(archived_blocks)) = query_archived_blocks(&func, args).await {
                return Ok(archived_blocks.blocks.into_iter().next());
            }
        }
        return Ok(None);
    }
    Err(String::from("Query block failed"))
}

pub fn sha256(input: &String) -> [u8; 32] {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().into()
}

pub async fn is_valid_signature(
    signature_hex: String,
    message: String,
    canister_id: CanisterId,
) -> bool {
    let public_key_hex = hex::encode(
        get_canister_public_key(canister_id)
            .await
            .expect("valid principal")
            .public_key,
    );
    let signature_bytes = hex::decode(&signature_hex).expect("failed to hex-decode signature");
    let pubkey_bytes = hex::decode(public_key_hex).expect("failed to hex-decode public key");
    let message_bytes = message.as_bytes();

    let signature = k256::ecdsa::Signature::try_from(signature_bytes.as_slice())
        .expect("failed to deserialize signature");
    k256::ecdsa::VerifyingKey::from_sec1_bytes(&pubkey_bytes)
        .expect("failed to deserialize sec1 encoding into public key")
        .verify(message_bytes, &signature)
        .is_ok()
}

pub async fn get_canister_public_key(
    canister_id: CanisterId,
) -> GenericResult<EcdsaPublicKeyResponse> {
    let request = EcdsaPublicKeyArgument {
        canister_id: Some(canister_id),
        derivation_path: vec![],
        key_id: EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id(),
    };

    let (res,) = ecdsa_public_key(request)
        .await
        .map_err(|e| format!("ecdsa_public_key failed {:?}", e))?;

    Ok(res)
}
