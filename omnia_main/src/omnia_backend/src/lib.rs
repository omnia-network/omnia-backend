mod http_endpoint;
mod manager;
mod user;
mod utils;

use candid::{candid_method, CandidType, Deserialize, Principal};
use ic_cdk::print;
use ic_cdk_macros::{init, post_upgrade};
use std::cell::RefCell;
use utils::update_database_principal;

// if you want to make the state persistent:
// - add serde::Serialize trait
// - implement pre_upgrade and post_upgrade as it's done in database canister
#[derive(Default, CandidType, Deserialize)]
struct State {
    pub database_principal: Option<Principal>,
}

thread_local! {
    /* flexible */ static STATE: RefCell<State>  = RefCell::new(State::default());
}

// to deploy this canister with the database principal id as init argument, use
// dfx deploy --argument '(null, "<database-canister-id>")'
// null first argument is needed to deploy internet_identity canister properly
#[init]
#[candid_method(init)]
fn init(_: Option<String>, arg2: String) {
    print("Init canister...");
    update_database_principal(arg2);
}

#[post_upgrade]
fn post_upgrade(_: Option<String>, arg2: String) {
    print("Post upgrade canister...");
    update_database_principal(arg2);
}

#[cfg(test)]
mod tests {
    use candid::export_service;
    use std::env;

    use omnia_types::affordance::*;
    use omnia_types::device::*;
    use omnia_types::environment::*;
    use omnia_types::errors::*;
    use omnia_types::gateway::*;
    use omnia_types::http::*;
    use omnia_types::updates::*;
    use omnia_types::virtual_persona::*;
    use std::collections::BTreeSet;

    #[test]
    fn generate_candid_interface() {
        use std::fs::write;
        let dir = env::current_dir().unwrap();
        let did_name = "omnia_backend.did";
        let did_path = dir.join(did_name);

        export_service!();
        write(did_path, __export_service()).expect("Write failed.");
    }
}
