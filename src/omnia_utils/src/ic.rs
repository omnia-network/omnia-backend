use candid::Principal;
use ic_ledger_types::{AccountIdentifier, Subaccount, Transaction};
use omnia_types::access_key::TransactionHash;
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
