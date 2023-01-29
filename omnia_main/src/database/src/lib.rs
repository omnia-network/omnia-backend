use std::collections::BTreeMap;
use std::{cell::RefCell, collections::BTreeSet, ops::Deref};

use candid::{CandidType, Deserialize, Principal};
use environment::StoredEnvironmentInfo;
use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk_macros::{post_upgrade, pre_upgrade};
use omnia_types::environment::EnvironmentUID;
use omnia_types::gateway::GatewayUID;
use profile::StoredUserProfile;
use serde::Serialize;

mod environment;
mod profile;
mod uuid;

#[derive(Default, CandidType, Serialize, Deserialize)]
struct State {
    pub user_profiles: BTreeMap<Principal, StoredUserProfile>,
    pub environments: BTreeMap<EnvironmentUID, StoredEnvironmentInfo>,
    pub initialized_gateways: BTreeSet<GatewayUID>,
}

impl State {
    fn default() -> Self {
        Self {
            user_profiles: BTreeMap::default(),
            environments: BTreeMap::default(),
            initialized_gateways: BTreeSet::default(),
        }
    }
}

thread_local! {
    /* stable */ static STATE: RefCell<State>  = RefCell::new(State::default());
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
