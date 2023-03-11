use std::collections::BTreeMap;
use std::{cell::RefCell, ops::Deref};

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk_macros::{post_upgrade, pre_upgrade};
use omnia_types::environment::{EnvironmentUID, Environment, Ip};
use omnia_types::gateway::{GatewayPrincipald, GatewayIp, GatewayPrincipalId, RegisteredGateway};
use omnia_types::http::{CanisterCallNonce, RequesterInfo};
use omnia_types::virtual_persona::VirtualPersona;
use serde::Serialize;

mod environment;
mod virtual_persona;
mod uuid;
mod auth;

#[derive(Default, CandidType, Serialize, Deserialize)]
struct State {
    pub virtual_personas: BTreeMap<Principal, VirtualPersona>,
    pub environments: BTreeMap<EnvironmentUID, Environment>,
    pub ip_to_env_uid: BTreeMap<Ip, EnvironmentUID>,
    pub registered_gateways: BTreeMap<GatewayPrincipalId, RegisteredGateway>,
    pub initialized_nonce_to_ip: BTreeMap<CanisterCallNonce, RequesterInfo>,
    pub initialized_gateways: BTreeMap<GatewayIp, GatewayPrincipald>,
}

impl State {
    fn default() -> Self {
        Self {
            virtual_personas: BTreeMap::default(),
            environments: BTreeMap::default(),
            ip_to_env_uid: BTreeMap::default(),
            registered_gateways: BTreeMap::default(),
            initialized_nonce_to_ip: BTreeMap::default(),
            initialized_gateways: BTreeMap::default(),
        }
    }

    pub fn create_virtual_persona(&mut self, virtual_persona_principal: Principal, new_virtual_persona: VirtualPersona) {
        self.virtual_personas.insert(virtual_persona_principal, new_virtual_persona);
    }
}

thread_local! {
    static STATE: RefCell<State>  = RefCell::new(State::default());
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| {
        ciborium::ser::into_writer(state.borrow().deref(), StableWriter::default())
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
        use omnia_types::environment::*;
        use omnia_types::gateway::*;
        use omnia_types::virtual_persona::*;
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::current_dir().unwrap());
        export_service!();
        write(dir.join("database.did"), __export_service()).expect("Write failed.");
    }
}
