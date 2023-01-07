use ic_cdk::export::Principal;
use std::cell::RefCell;
use std::collections::BTreeMap;

mod utils;
mod store_types;

use store_types as StoreTypes;

// USER PROFILE DATABASE
type UserProfileStore = BTreeMap<Principal, StoreTypes::UserProfile>;

thread_local! {
    static USER_PROFILE_STORE: RefCell<UserProfileStore> = RefCell::default();
}