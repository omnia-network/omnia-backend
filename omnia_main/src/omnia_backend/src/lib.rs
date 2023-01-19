use candid::parser::types::PrimType;
use candid::Principal;
use getrandom::register_custom_getrandom;
use omnia_types::gateway::GatewayUID;
use std::cell::RefCell;
use std::collections::BTreeSet;

register_custom_getrandom!(custom_getrandom);

fn custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    // TODO get some randomness
    return Ok(());
}

type InitializedGatewayStore = BTreeSet<GatewayUID>;

// TODO: use this to share the Database Principal with other modules
// struct DatabasePrincipal {
//     principal: Principal,
// }

thread_local! {
    static INITIALIZED_GATEWAY_STORE: RefCell<InitializedGatewayStore> = RefCell::default();
    // initialize principal to current principal just to avoid Option in the struct
    // static DATABASE_PRINCIPAL: RefCell<DatabasePrincipal> = RefCell::new(DatabasePrincipal { principal: ic_cdk::id() });
}

// TODO: make the RefCell assignment work to share the Database canister Principal with all other modules
// to deploy this canister with the database principal id as init argument, use
// dfx deploy omnia_backend --argument '("rrkah-fqaaa-aaaaa-aaaaq-cai")'
// and change the id accordingly
// #[ic_cdk_macros::init]
// async fn initialize() {
//     let call_arg = ic_cdk::api::call::arg_data::<(Option<String>,)>().0;

//     match call_arg {
//         Some(database_canister_id) => {
//             let remote_principal: Principal = Principal::from_text(database_canister_id).unwrap();
//             DATABASE_PRINCIPAL.with(|database_principal| {
//                 database_principal.borrow_mut().principal = remote_principal;
//             });
//         }
//         None => ic_cdk::print(format!("Init: {:?}", call_arg)),
//     }
// }

mod manager;
mod user;
