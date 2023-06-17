use candid::Principal;
use ic_ledger_types::{AccountIdentifier, Subaccount};

pub fn principal_to_account(principal: Principal) -> AccountIdentifier {
    AccountIdentifier::new(&principal, &Subaccount([0; 32]))
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
