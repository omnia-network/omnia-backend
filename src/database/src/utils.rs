use candid::Principal;
use ic_cdk::{caller, print, trap};

use crate::STATE;

pub fn caller_is_omnia_backend() {
    let caller = caller();

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
