use candid::Principal;
use ic_cdk::api::{management_canister::provisional::CanisterId, print, time};
use ic_ledger_types::{
    transfer, AccountIdentifier, BlockIndex, Memo, Subaccount, Timestamp, Tokens, Transaction,
    TransferArgs, DEFAULT_FEE, DEFAULT_SUBACCOUNT,
};
use omnia_types::{access_key::TransactionHash, errors::GenericResult};
use sha2::Sha256;

pub fn principal_to_account(principal: Principal) -> AccountIdentifier {
    AccountIdentifier::new(&principal, &Subaccount([0; 32]))
}

/// There's no reference in the IC docs regarding how to obtain the transaction hash.
///
/// The only poor references are:
/// - https://internetcomputer.org/docs/current/references/ledger#_chaining_ledger_blocks
/// - https://github.com/dfinity/ic/blob/3269d80bf263aed8cb829478ad37ac7f563b42cb/rs/rosetta-api/icp_ledger/src/lib.rs#L229
///
/// TODO: since serde_cbor is unmaintained, use ciborium instead.
pub fn get_transaction_hash(transaction: Transaction) -> TransactionHash {
    use sha2::Digest;
    let mut state = Sha256::new();
    state.update(&serde_cbor::ser::to_vec_packed(&transaction).unwrap());
    state.finalize().into()
}

pub async fn transfer_to(
    ledger_canister_id: CanisterId,
    principal: Principal,
    amount: Tokens,
) -> GenericResult<BlockIndex> {
    let block_index = transfer(
        ledger_canister_id,
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
    .map_err(|e| format!("call to ledger failed: {:?}", e))?
    .map_err(|e| format!("transfer failed: {:?}", e))?;

    print(format!(
        "Created block with index: {:?}, transferred: {:?} to principal ID: {:?}",
        block_index,
        amount,
        principal.to_string()
    ));

    Ok(block_index)
}

#[cfg(test)]
mod tests {
    use candid::Principal;

    use crate::ic::principal_to_account;

    #[test]
    fn test_principal_to_account() {
        let principal = Principal::from_text("bd3sg-teaaa-aaaaa-qaaba-cai").unwrap();
        let account = principal_to_account(principal);
        assert_eq!(
            account.to_string(),
            "3dd5d9a74d6bfd1e3d96f75eef3c2ae712b22d23600607c91747abc8a2d2d6a4"
        );
    }
}
