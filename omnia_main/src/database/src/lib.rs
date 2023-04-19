use std::collections::BTreeSet;
use std::{cell::RefCell, ops::Deref};
use candid::{CandidType, Deserialize};
use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk_macros::{post_upgrade, pre_upgrade};
use omnia_types::CrudMap;
use omnia_types::affordance::AffordanceValue;
use omnia_types::device::{RegisteredDeviceIndex, RegisteredDeviceValue, DeviceUid};
use omnia_types::environment::{EnvironmentIndex, EnvironmentValue, EnvironmentUidValue, EnvironmentUidIndex};
use omnia_types::gateway::{RegisteredGatewayValue, InitializedGatewayValue, InitializedGatewayIndex, RegisteredGatewayIndex};
use omnia_types::http::{IpChallengeValue, IpChallengeIndex};
use omnia_types::virtual_persona::{VirtualPersonaIndex, VirtualPersonaValue};
use omnia_types::updates::{UpdateIndex, UpdateValue};
use serde::Serialize;

mod environment;
mod virtual_persona;
mod uuid;
mod auth;

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
    pub affordance_devices_index: CrudMap<AffordanceValue, BTreeSet<DeviceUid>>,
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
            affordance_devices_index: CrudMap::default(),
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
    use candid::{
        export_service,
        utils::{service_compatible, CandidSource},
    };
    use std::env;

    use super::*;
    use omnia_types::device::*;
    use omnia_types::environment::*;
    use omnia_types::gateway::*;
    use omnia_types::user::*;

    #[test]
    fn check_candid_interface() {
        let dir = env::current_dir().unwrap();
        let did_name = "database.did";
        let did_path = dir.join(did_name);

        export_service!();
        let new_interface = __export_service();

        service_compatible(
            CandidSource::Text(&new_interface),
            CandidSource::File(&did_path),
        )
        .unwrap();
    }
}
