use std::collections::BTreeMap;
use std::{cell::RefCell, ops::Deref};

use candid::{CandidType, Deserialize, Principal};
use environment::StoredEnvironmentInfo;
use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk_macros::{post_upgrade, pre_upgrade};
use omnia_types::environment::EnvironmentUID;
use omnia_types::gateway::{GatewayUID, GatewayIp};
use omnia_types::http::{CanisterCallNonce, RequesterInfo};
use omnia_types::user::VirtualPersona;
use serde::Serialize;

mod environment;
mod profile;
mod uuid;
mod auth;

#[derive(Default, CandidType, Serialize, Deserialize)]
struct State {
    pub virtual_personas: BTreeMap<Principal, VirtualPersona>,
    pub environments: BTreeMap<EnvironmentUID, StoredEnvironmentInfo>,
    pub initialized_nonce_to_ip: BTreeMap<CanisterCallNonce, RequesterInfo>,
    pub initialized_gateways: BTreeMap<GatewayIp, GatewayUID>,
}

impl State {
    fn default() -> Self {
        Self {
            virtual_personas: BTreeMap::default(),
            environments: BTreeMap::default(),
            initialized_nonce_to_ip: BTreeMap::default(),
            initialized_gateways: BTreeMap::default(),
        }
    }
}

thread_local! {
    static STATE: RefCell<State>  = RefCell::new(State::default());
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|cell| {
        ciborium::ser::into_writer(cell.borrow().deref(), StableWriter::default())
            .expect("failed to encode state")
    })
}

#[post_upgrade]
fn post_upgrade() {
    STATE.with(|cell| {
        *cell.borrow_mut() =
            ciborium::de::from_reader(StableReader::default()).expect("failed to decode state");
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::export_service;

    #[test]
    fn save_candid() {
        use omnia_types::device::*;
        use omnia_types::environment::*;
        use omnia_types::gateway::*;
        use omnia_types::user::*;
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::current_dir().unwrap());
        export_service!();
        write(dir.join("database.did"), __export_service()).expect("Write failed.");
    }
}
