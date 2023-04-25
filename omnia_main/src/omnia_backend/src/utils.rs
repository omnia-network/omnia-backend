use candid::Principal;
use ic_cdk::print;
use omnia_types::utils::RdfDatabaseConnection;

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

pub fn get_rdf_database_connection() -> RdfDatabaseConnection {
    STATE
        .with(|state| state.borrow().rdf_database_connection.clone())
        .expect("No RDF database connection")
}

pub fn update_rdf_database_connection(rdf_database_base_url: String, rdf_database_api_key: String) {
    let rdf_database_connection = RdfDatabaseConnection {
        base_url: rdf_database_base_url,
        api_key: rdf_database_api_key,
    };

    print(format!("RDF Database connection: {:?}", rdf_database_connection));

    STATE.with(|state| {
        state.borrow_mut().rdf_database_connection = Some(rdf_database_connection);
    });
}
