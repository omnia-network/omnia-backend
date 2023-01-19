use ic_cdk::export::Principal;
use std::cell::RefCell;
use std::collections::BTreeMap;

mod store_types;
mod utils;

use store_types::StoredUserProfile;

// USER PROFILE DATABASE
type UserProfileStore = BTreeMap<Principal, StoredUserProfile>;

thread_local! {
    static USER_PROFILE_STORE: RefCell<UserProfileStore> = RefCell::default();
}
