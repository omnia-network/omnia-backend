use candid::Principal;
use omnia_types::gateway::GatewayUID;
use std::cell::RefCell;
use std::collections::BTreeSet;

type InitializedGatewayStore = BTreeSet<GatewayUID>;

pub struct DatabasePrincipal {
    pub principal: Option<Principal>,
}

thread_local! {
    pub static INITIALIZED_GATEWAY_STORE: RefCell<InitializedGatewayStore> = RefCell::default();
    // initialize principal to current principal just to avoid Option in the struct
    pub static DATABASE_PRINCIPAL: RefCell<DatabasePrincipal> = RefCell::new(DatabasePrincipal { principal: None });
}
