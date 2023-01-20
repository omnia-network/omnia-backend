use std::collections::BTreeMap;
use std::{cell::RefCell, ops::Deref};

use candid::{CandidType, Deserialize, Principal};
use environment::StoredEnvironmentInfo;
use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk_macros::{post_upgrade, pre_upgrade};
use omnia_types::environment::EnvironmentUID;
use profile::StoredUserProfile;
use serde::Serialize;

mod environment;
mod profile;
mod uuid;

#[derive(Default, CandidType, Serialize, Deserialize)]
struct State {
    pub user_profiles: BTreeMap<Principal, StoredUserProfile>,
    pub environments: BTreeMap<EnvironmentUID, StoredEnvironmentInfo>,
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
