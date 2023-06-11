use candid::Principal;
use ic_cdk::print;
use ic_ledger_types::{
    account_balance, transfer, AccountBalanceArgs, AccountIdentifier, BlockIndex, Memo, Tokens,
    TransferArgs, DEFAULT_FEE, DEFAULT_SUBACCOUNT,
};

use crate::STATE;

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

    print(format!("Caller's balance: {:?}", balance));

    balance
}

pub async fn transfer_to(principal: Principal) -> BlockIndex {
    let ledger_principal = STATE
        .with(|state| state.borrow().ledger_principal)
        .expect("should have provided ledger principal id");

    transfer(
        ledger_principal,
        TransferArgs {
            memo: Memo(0),
            amount: Tokens::from_e8s(1_000_000),
            fee: DEFAULT_FEE,
            from_subaccount: None,
            to: AccountIdentifier::new(&principal, &DEFAULT_SUBACCOUNT),
            created_at_time: None,
        },
    )
    .await
    .expect("call to ledger failed")
    .expect("transfer failed")
}
