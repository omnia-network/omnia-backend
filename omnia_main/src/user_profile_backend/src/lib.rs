use ic_cdk::{
    api::call::ManualReply,
    export::{
        candid::{CandidType, Deserialize},
        Principal,
    },
};
use std::cell::RefCell;
use std::collections::BTreeMap;

type ProfileStore = BTreeMap<Principal, Profile>;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Profile {
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
}

thread_local! {
    static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
}

#[ic_cdk_macros::query(name = "getSelf", manual_reply = true)]
fn get_self(principal_str: String) -> ManualReply<Profile> {
    let principal = Principal::from_text(principal_str).unwrap();
    PROFILE_STORE.with(|profile_store| {
        if let Some(profile) = profile_store.borrow().get(&principal) {
            ManualReply::one(profile)
        } else {
            ManualReply::one(Profile::default())
        }
    })
}

#[ic_cdk_macros::update(name = "updateProfile")]
fn update(principal_str: String, profile: Profile) {
    let principal = Principal::from_text(principal_str).unwrap();
    ic_cdk::print(format!("Updating profile: {:?} of user: {:?}", profile, principal));
    PROFILE_STORE.with(|profile_store| {
        profile_store.borrow_mut().insert(principal, profile);
    });
}

#[ic_cdk_macros::query(name = "userIsInEnvironment")]
fn user_is_in_environment(principal_str: String) -> bool {
    let principal = Principal::from_text(principal_str).unwrap();
    ic_cdk::print(format!("Checking if user: {:?} is already in environment", principal));
    PROFILE_STORE.with(|profile_store| {
        profile_store.borrow().contains_key(&principal)
    })
}