use std::collections::BTreeSet;
use std::{cell::RefCell, ops::Deref};
use candid::{CandidType, Deserialize};
use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk_macros::{post_upgrade, pre_upgrade};
use omnia_types::CrudMap;
use omnia_types::affordance::AffordanceValue;
use omnia_types::device::{RegisteredDeviceIndex, RegisteredDeviceValue, DeviceUid};
use omnia_types::environment::{EnvironmentIndex, EnvironmentValue, EnvironmentUidValue, EnvironmentUidIndex, EnvironmentUID};
use omnia_types::errors::GenericResult;
use omnia_types::gateway::{RegisteredGatewayValue, InitializedGatewayValue, InitializedGatewayIndex, RegisteredGatewayIndex};
use omnia_types::http::{IpChallengeValue, IpChallengeIndex, Ip, IpChallengeNonce};
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

    pub fn get_environment_uid_by_ip(&self, ip: Ip) -> GenericResult<EnvironmentUID> {
        let environment_uid_index = EnvironmentUidIndex {
            ip,
        };
        let environment_uid_value = self.environment_uids.read(&environment_uid_index)?.clone();
        Ok(environment_uid_value.env_uid)
    }

    pub fn validate_ip_challenge_by_nonce(&mut self, nonce: IpChallengeNonce) -> GenericResult<IpChallengeValue> {
        let ip_challenge_index = IpChallengeIndex {
            nonce,
        };
        let ip_challenge_value = self.ip_challenges.validate_ip_challenge(&ip_challenge_index)?;
        Ok(ip_challenge_value)
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
    use candid::export_service;

    #[test]
    fn save_candid() {
        use omnia_types::environment::*;
        use omnia_types::gateway::*;
        use omnia_types::http::*;
        use omnia_types::errors::*;
        use omnia_types::virtual_persona::*;
        use omnia_types::updates::*;
        use omnia_types::device::*;
        use omnia_types::affordance::*;
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;
        use std::collections::BTreeSet;

        let dir = PathBuf::from(env::current_dir().unwrap());
        export_service!();
        write(dir.join("database.did"), __export_service()).expect("Write failed.");
    }
}
