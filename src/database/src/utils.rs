use candid::Principal;
use ic_cdk::{caller, print, trap};
use ic_ledger_types::{
    account_balance, AccountBalanceArgs, AccountIdentifier, Tokens, DEFAULT_SUBACCOUNT,
};

use crate::STATE;

pub async fn check_callers_balance() -> Tokens {
    let ledger_principal = STATE
        .with(|state| state.borrow().ledger_principal)
        .expect("should have provided ledger principal id");
    let balance = account_balance(
        ledger_principal,
        AccountBalanceArgs {
            account: AccountIdentifier::new(
                &Principal::from_text(
                    "ygoe7-xpj6n-24gsd-zksfw-2mywm-xfyop-yvlsp-ctlwa-753xv-wz6rk-uae",
                )
                .unwrap(),
                &DEFAULT_SUBACCOUNT,
            ),
        },
    )
    .await
    .expect("call to ledger failed");

    print(format!("Caller's balance: {:?}", balance));

    balance
}

pub fn caller_is_omnia_backend() {
    let caller = caller();
    print(format!("Caller with principal: {:?}", caller.to_string()));

    STATE.with(|state| {
        if caller
            != state
                .borrow()
                .omnia_backend_principal
                .expect("should have provided omnia_backend principal id")
        {
            trap("only omnia_backend can call database")
        }
    })
}

pub fn update_omnia_backend_principal(omnia_backend_canister_principal_id: String) {
    print(format!(
        "Omnia Backend canister Principal ID: {:?}",
        omnia_backend_canister_principal_id
    ));

    let remote_principal: Principal = Principal::from_text(omnia_backend_canister_principal_id)
        .expect("Invalid Omnia Backend canister principal id");
    STATE.with(|state| {
        state.borrow_mut().omnia_backend_principal = Some(remote_principal);
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
