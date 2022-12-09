use ic_cdk::{
    api::call::ManualReply,
    export::{
        candid::{CandidType, Deserialize},
        Principal,
    },
};
use std::cell::RefCell;
use std::collections::BTreeMap;

type EnvironmentStore = BTreeMap<Principal, EnvironmentInfo>;

#[derive(CandidType)]
struct EnvironmentInfo {
    pub env_name: String,
    pub env_uid: String,
}

#[derive(Debug, CandidType, Deserialize)]
struct EnvironmentRegistrationInput {
    pub env_name: String,
}

#[derive(Debug, CandidType, Deserialize)]
struct EnvironmentRegistrationResult {
    pub env_uid: String,
}

thread_local! {
    static ENVIRONMENT_STORE: RefCell<EnvironmentStore> = RefCell::default();
}

#[ic_cdk_macros::update(name = "InitializeNewEnvironment", manual_reply = true)]
fn initialize_new_environment(
    environment_manager_principal_id: String,
    environment_registration_input: EnvironmentRegistrationInput
) -> ManualReply<EnvironmentRegistrationResult>{

    let environment_manager_principal = Principal::from_text(environment_manager_principal_id).unwrap();
    ic_cdk::print(format!("Initializing environment: {:?} managed by: {:?}", environment_registration_input, environment_manager_principal));

    ENVIRONMENT_STORE.with(|environment_store| {
        environment_store.borrow_mut().insert(
            environment_manager_principal,
            EnvironmentInfo {
                env_name: String::from("Random environment name"),
                env_uid: String::from("Random environment UID"),
            }
        );
    });

    let environment_registration_result = EnvironmentRegistrationResult {
        env_uid: String::from("Random environment UID"),
    };

    ManualReply::one(environment_registration_result)
}