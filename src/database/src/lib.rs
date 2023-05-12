use candid::{CandidType, Deserialize};
use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk_macros::{post_upgrade, pre_upgrade};
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
use serde::Serialize;
use std::{cell::RefCell, ops::Deref};

mod auth;
mod environment;
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
        }
    }
}

thread_local! {
    /* stable */ static STATE: RefCell<State>  = RefCell::new(State::default());
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
    use candid::export_service;
    use std::env;

    use super::*;
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
