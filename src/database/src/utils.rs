use candid::Principal;
use ic_cdk::{caller, print, trap};

use crate::OMNIA_BACKEND_PRINCIPAL;

pub fn caller_is_omnia_backend() {
    let caller = caller();

    OMNIA_BACKEND_PRINCIPAL.with(|state| {
        if caller
            != state
                .borrow()
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
    OMNIA_BACKEND_PRINCIPAL.with(|state| {
        *state.borrow_mut() = Some(remote_principal);
    });
}
