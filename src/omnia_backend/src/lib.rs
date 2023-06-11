mod http_endpoint;
mod manager;
mod outcalls;
mod random;
mod rdf;
mod user;
mod utils;

use candid::{candid_method, CandidType, Deserialize, Principal};
use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk::print;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade};
use ic_oxigraph::io::GraphFormat;
use ic_oxigraph::model::GraphNameRef;
use ic_oxigraph::store::Store;
use rand::{rngs::StdRng, SeedableRng};
use random::init_rng;
use std::cell::RefCell;
use utils::{update_database_principal, update_ledger_principal};

// if you want to make the state persistent:
// - add serde::Serialize trait
// - implement pre_upgrade and post_upgrade as it's done in database canister
#[derive(Default, CandidType, Deserialize)]
struct State {
    pub database_principal: Option<Principal>,
    pub ledger_principal: Option<Principal>,
}

impl State {
    fn default() -> Self {
        Self {
            database_principal: None,
            ledger_principal: None,
        }
    }
}

thread_local! {
    /* flexible */ static STATE: RefCell<State>  = RefCell::new(State::default());
    /* flexible */ static RNG_REF_CELL: RefCell<StdRng> = RefCell::new(SeedableRng::from_seed([0_u8; 32]));
    /* stable */ static RDF_DB: RefCell<Store>  = RefCell::new(Store::new().unwrap());
}

// to deploy this canister with the database principal id as init argument, use
// dfx deploy --argument '(null, "<database-canister-id>")'
// null first argument is needed to deploy internet_identity canister properly
#[init]
#[candid_method(init)]
fn init(
    _omnia_backend_canister_principal_id: String,
    database_canister_principal_id: String,
    ledger_canister_principal_id: String,
) {
    print("Init canister...");

    // initialize rng
    init_rng();

    // initialize rng in the ic-oxigraph library
    RNG_REF_CELL.with(ic_oxigraph::init);

    update_database_principal(database_canister_principal_id);
    update_ledger_principal(ledger_canister_principal_id);
}

#[pre_upgrade]
fn pre_upgrade() {
    RDF_DB.with(|store| {
        let mut buffer = Vec::new();
        store
            .borrow()
            .dump_graph(
                &mut buffer,
                GraphFormat::NTriples,
                GraphNameRef::DefaultGraph,
            )
            .expect("failed to dump RDF graph");

        ciborium::ser::into_writer(buffer.as_slice(), StableWriter::default())
            .expect("failed to encode state")
    });
}

#[post_upgrade]
fn post_upgrade(
    _omnia_backend_canister_principal_id: String,
    database_canister_principal_id: String,
    ledger_canister_principal_id: String,
) {
    print("Post upgrade canister...");

    // initialize rng
    init_rng();

    // initialize rng in the ic-oxigraph library
    RNG_REF_CELL.with(ic_oxigraph::init);

    update_database_principal(database_canister_principal_id);

    RDF_DB.with(|cell| {
        let deserialized: Vec<u8> =
            ciborium::de::from_reader(StableReader::default()).expect("failed to decode state");

        let store = Store::new().unwrap();
        // loading the graph can probably be optimized
        store
            .load_graph(
                deserialized.as_slice(),
                GraphFormat::NTriples,
                GraphNameRef::DefaultGraph,
                None,
            )
            .unwrap();

        *cell.borrow_mut() = store;
    });

    update_ledger_principal(ledger_canister_principal_id);
}

#[cfg(test)]
mod tests {
    use candid::export_service;
    use std::env;

    use omnia_types::device::*;
    use omnia_types::environment::*;
    use omnia_types::errors::*;
    use omnia_types::gateway::*;
    use omnia_types::http::*;
    use omnia_types::updates::*;
    use omnia_types::virtual_persona::*;

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
