use candid::{candid_method, CandidType, Deserialize, Principal};
use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade};
use omnia_types::access_key::{AccessKeyIndex, AccessKeyValue};
use omnia_types::device::{RegisteredDeviceIndex, RegisteredDeviceValue};
use omnia_types::environment::{
    EnvironmentIndex, EnvironmentUidIndex, EnvironmentUidValue, EnvironmentValue,
};
use omnia_types::gateway::{
    InitializedGatewayIndex, InitializedGatewayValue, RegisteredGatewayIndex,
    RegisteredGatewayValue,
};
use omnia_types::http::{IpChallengeIndex, IpChallengeValue};
use omnia_types::updates::{UpdateIndex, UpdateValue};
use omnia_types::virtual_persona::{VirtualPersonaIndex, VirtualPersonaValue};
use omnia_types::CrudMap;
use omnia_utils::random::init_rng;
use serde::Serialize;
use std::{cell::RefCell, ops::Deref};
use utils::update_omnia_backend_principal;

mod access_key;
mod auth;
mod environment;
mod utils;
mod virtual_persona;

#[derive(Default, CandidType, Serialize, Deserialize)]
struct State {
    pub virtual_personas: CrudMap<VirtualPersonaIndex, VirtualPersonaValue>,
    pub environments: CrudMap<EnvironmentIndex, EnvironmentValue>,
    pub environment_uids: CrudMap<EnvironmentUidIndex, EnvironmentUidValue>,
    pub registered_gateways: CrudMap<RegisteredGatewayIndex, RegisteredGatewayValue>,
    pub ip_challenges: CrudMap<IpChallengeIndex, IpChallengeValue>,
    pub initialized_gateways: CrudMap<InitializedGatewayIndex, InitializedGatewayValue>,
    pub updates: CrudMap<UpdateIndex, UpdateValue>,
    pub registered_devices: CrudMap<RegisteredDeviceIndex, RegisteredDeviceValue>,
    pub valid_access_keys: CrudMap<AccessKeyIndex, AccessKeyValue>,
    pub omnia_backend_principal: Option<Principal>,
}

impl State {
    fn default() -> Self {
        Self {
            virtual_personas: CrudMap::default(),
            environments: CrudMap::default(),
            environment_uids: CrudMap::default(),
            registered_gateways: CrudMap::default(),
            ip_challenges: CrudMap::default(),
            initialized_gateways: CrudMap::default(),
            updates: CrudMap::default(),
            registered_devices: CrudMap::default(),
            valid_access_keys: CrudMap::default(),
            omnia_backend_principal: None,
        }
    }
}

thread_local! {
    /* stable */ static STATE: RefCell<State>  = RefCell::new(State::default());
}

#[init]
#[candid_method(init)]
fn init(
    omnia_backend_canister_principal_id: String,
    _database_canister_principal_id: String,
    _ledger_canister_principal_id: String,
) {
    // initialize rng
    init_rng();

    update_omnia_backend_principal(omnia_backend_canister_principal_id);
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| {
        ciborium::ser::into_writer(state.borrow().deref(), StableWriter::default())
            .expect("failed to encode state")
    });
}

#[post_upgrade]
fn post_upgrade(
    omnia_backend_canister_principal_id: String,
    _database_canister_principal_id: String,
    _ledger_canister_principal_id: String,
) {
    // initialize rng
    init_rng();

    STATE.with(|cell| {
        *cell.borrow_mut() =
            ciborium::de::from_reader(StableReader::default()).expect("failed to decode state");
    });

    update_omnia_backend_principal(omnia_backend_canister_principal_id);
}

#[cfg(test)]
mod tests {
    use candid::export_service;
    use std::env;

    use super::*;
    use omnia_types::access_key::*;
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
        let did_name = "database.did";
        let did_path = dir.join(did_name);

        export_service!();
        write(did_path, __export_service()).expect("Write failed.");
    }
}
