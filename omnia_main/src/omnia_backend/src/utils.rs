use candid::Principal;

use crate::stores::DATABASE_PRINCIPAL;

pub fn get_database_principal() -> Principal {
    DATABASE_PRINCIPAL
        .with(|database_principal| database_principal.borrow().principal)
        .expect("No Database canister principal")
}

pub fn update_database_principal(database_principal_id: Option<String>) {
    match database_principal_id {
        Some(id) => {
            ic_cdk::print(format!("Database Principal ID: {:?}", id));

            let remote_principal: Principal = Principal::from_text(id).expect("Invalid Database canister principal id");
            DATABASE_PRINCIPAL.with(|database_principal| {
                database_principal.borrow_mut().principal = Some(remote_principal);
            });
        }
        None => {
            ic_cdk::print(format!("No Database canister principal ID to update"));
        }
    }
}
