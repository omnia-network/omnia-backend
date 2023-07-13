use candid::{candid_method, Principal};
use ic_cdk_macros::{init, post_upgrade};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::DefaultMemoryImpl;
use omnia_core_sdk::random::init_rng;
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
use std::cell::RefCell;
use utils::update_omnia_backend_principal;

mod access_key;
mod auth;
mod environment;
mod utils;
mod virtual_persona;

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
}

impl State {
    fn default() -> Self {
        Self {
            virtual_personas: CrudMap::default(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
            ),
            environments: CrudMap::default(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
            ),
            environment_uids: CrudMap::default(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
            ),
            registered_gateways: CrudMap::default(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))),
            ),
            ip_challenges: CrudMap::default(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))),
            ),
            initialized_gateways: CrudMap::default(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))),
            ),
            updates: CrudMap::default(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6)))),
            registered_devices: CrudMap::default(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7))),
            ),
            valid_access_keys: CrudMap::default(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(8))),
            ),
        }
    }
}

thread_local! {
    /* stable */ static STATE: RefCell<State> = RefCell::new(State::default());
    /* flexible */ static OMNIA_BACKEND_PRINCIPAL: RefCell<Option<Principal>> = RefCell::new(None);
    /* flexible */ static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
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

#[post_upgrade]
fn post_upgrade(
    omnia_backend_canister_principal_id: String,
    _database_canister_principal_id: String,
    _ledger_canister_principal_id: String,
) {
    // initialize rng
    init_rng();

    update_omnia_backend_principal(omnia_backend_canister_principal_id);
}

#[cfg(test)]
mod tests {
    use candid::export_service;
    use std::env;

    use super::*;
    use omnia_core_sdk::access_key::UniqueAccessKey;
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
