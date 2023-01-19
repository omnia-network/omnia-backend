use std::cell::RefCell;
use std::collections::BTreeMap;

pub mod store_types;
pub mod utils;

use omnia_types::environment::EnvironmentUID;
use store_types::StoredEnvironmentInfo;

//  ENVIRONMENTS DATABASE
type EnvironmentStore = BTreeMap<EnvironmentUID, StoredEnvironmentInfo>;

thread_local! {
    static ENVIRONMENT_STORE: RefCell<EnvironmentStore> = RefCell::default();
}
