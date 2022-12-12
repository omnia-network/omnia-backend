use ic_cdk::api::call::ManualReply;
use candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use std::cell::RefCell;
use std::collections::BTreeMap;

//  ENVIRONMENTS DATABASE
type EnvironmentStore = BTreeMap<Principal, EnvironmentInfo>;

#[derive(Debug, CandidType)]
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

// USER PROFILE DATABASE
type UserProfileStore = BTreeMap<Principal, UserProfile>;

#[derive(Clone, Debug, CandidType)]
struct UserProfile {
    pub user_principal_id: String,
    pub environment_uid: Option<String>,
}

thread_local! {
    static USER_PROFILE_STORE: RefCell<UserProfileStore> = RefCell::default();
    static ENVIRONMENT_STORE: RefCell<EnvironmentStore> = RefCell::default();
}



#[ic_cdk_macros::update(name = "initializeNewEnvironment", manual_reply = true)]
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



#[ic_cdk_macros::update(name = "setUserInEnvironment", manual_reply = true)]
fn set_user_in_environment(user_principal_id: String, env_uid: String) -> ManualReply<EnvironmentInfo> {

    let user_principal = Principal::from_text(user_principal_id).unwrap();

    match get_user_profile_if_exists(user_principal) {
        Some(user_profile) => {
            let updated_user_profile = UserProfile {
                environment_uid : Some(env_uid.clone()),
                ..user_profile
            };

            USER_PROFILE_STORE.with(|profile_store| {
                profile_store.borrow_mut().insert(user_principal, updated_user_profile);
            });

            ic_cdk::print(format!("User: {:?} set in environment with UID: {:?}", user_principal, env_uid));

            ManualReply::one(EnvironmentInfo {
                env_name: String::from("Fake environment name"),
                env_uid,
            })

        },
        None => {
            panic!("User does not have a profile")
        }
    }

}


#[ic_cdk_macros::update(name = "getUserProfile", manual_reply = true)]
fn get_user_profile(user_principal_id: String) -> ManualReply<UserProfile> {

    let user_principal = Principal::from_text(user_principal_id).unwrap();

    match get_user_profile_if_exists(user_principal) {
        Some(user_profile) => {
            ic_cdk::print(format!("User: {:?} has profile: {:?}", user_principal, user_profile));
            ManualReply::one(user_profile)
        },
        None => {
            ic_cdk::print("User does not have a profile");

            // create new user profile
            let new_user_profile = UserProfile {
                user_principal_id: user_principal.to_string(),
                environment_uid: None,
            };
    
            ic_cdk::print(format!("Created profile: {:?} of user: {:?}", new_user_profile, user_principal));

            USER_PROFILE_STORE.with(|profile_store| {
                profile_store.borrow_mut().insert(user_principal, new_user_profile.clone());
            });

            ManualReply::one(new_user_profile)
        }
    }
}

fn get_user_profile_if_exists(user_principal: Principal) -> Option<UserProfile> {
    USER_PROFILE_STORE.with(|profile_store| {
        match profile_store.borrow().get(&user_principal) {
            Some(user_profile) => Some(user_profile.to_owned()),
            None => None
        }
    })
}