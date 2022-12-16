use ic_cdk::api::call::ManualReply;
use candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use std::cell::RefCell;
use std::collections::BTreeMap;
use getrandom::register_custom_getrandom;
use rand::Rng;

type PrincipalId = String;
type EnvironmentUID = u32;
type GatewayUID = u32;
type DeviceUID = u32;

//  ENVIRONMENTS DATABASE
type EnvironmentStore = BTreeMap<EnvironmentUID, EnvironmentInfo>;



#[derive(Debug, Clone, CandidType, Deserialize)]
struct DeviceInfo {
    pub device_name: String,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
struct GatewayInfo {
    pub gateway_name: String,
    pub gateway_uid: GatewayUID,
    pub devices: BTreeMap<DeviceUID, DeviceInfo>,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
struct EnvironmentInfo {
    pub env_name: String,
    pub env_uid: EnvironmentUID,
    pub env_gateways: BTreeMap<GatewayUID, GatewayInfo>,
    pub env_manager_principal_id: PrincipalId,
}

#[derive(Debug, CandidType, Deserialize)]
struct EnvironmentCreationInput {
    pub env_name: String,
}

#[derive(Debug, CandidType, Deserialize)]
struct EnvironmentCreationResult {
    pub env_name: String,
    pub env_uid: EnvironmentUID,
}

#[derive(Debug, CandidType, Deserialize)]
struct GatewayRegistrationInput {
    pub env_uid: EnvironmentUID,
    pub gateway_name: String,
}

#[derive(Debug, CandidType, Deserialize)]
struct GatewayRegistrationResult {
    pub gateway_name: String,
    pub gateway_uid: GatewayUID,
}

#[derive(Debug, CandidType, Deserialize)]
struct DeviceRegistrationInput {
    pub env_uid: EnvironmentUID,
    pub gateway_uid: GatewayUID,
    pub device_name: String,
}

#[derive(Debug, CandidType, Deserialize)]
struct DeviceRegistrationResult {
    pub device_name: String,
    pub device_uid: DeviceUID,
}

// USER PROFILE DATABASE
type UserProfileStore = BTreeMap<Principal, UserProfile>;

#[derive(Clone, Debug, CandidType)]
struct UserProfile {
    pub user_principal_id: PrincipalId,
    pub environment_uid: Option<EnvironmentUID>,
}

thread_local! {
    static USER_PROFILE_STORE: RefCell<UserProfileStore> = RefCell::default();
    static ENVIRONMENT_STORE: RefCell<EnvironmentStore> = RefCell::default();
}



#[ic_cdk_macros::update(name = "createNewEnvironment", manual_reply = true)]
fn create_new_environment(
    environment_manager_principal_id: PrincipalId,
    environment_creation_input: EnvironmentCreationInput
) -> ManualReply<EnvironmentCreationResult> {

    ic_cdk::print(format!("Creating new environment: {:?} managed by: {:?}", environment_creation_input, environment_manager_principal_id));

    register_custom_getrandom!(custom_getrandom);
    let environment_uid = rand::thread_rng().gen_range(0..100);
    ic_cdk::print(format!("Environment UID: {:?}", environment_uid));

    ENVIRONMENT_STORE.with(|environment_store| {
        environment_store.borrow_mut().insert(
            environment_uid,
            EnvironmentInfo {
                env_name: environment_creation_input.env_name.clone(),
                env_uid: environment_uid,
                env_gateways: BTreeMap::new(),
                env_manager_principal_id: environment_manager_principal_id,
            }
        );
    });

    let environment_creation_result = EnvironmentCreationResult {
        env_name: environment_creation_input.env_name,
        env_uid: environment_uid,
    };

    ManualReply::one(environment_creation_result)
}



#[ic_cdk_macros::update(name = "registerGatewayInEnvironment", manual_reply = true)]
fn register_gateway_in_environment(
    environment_manager_principal_id: PrincipalId,
    gateway_registration_input: GatewayRegistrationInput
) -> ManualReply<GatewayRegistrationResult> {

    match get_environment_info_by_uid(&gateway_registration_input.env_uid) {
        Some(mut environment_info) => {
            ic_cdk::print(format!("Registering gateway {:?} in environment with UID: {:?} managed by: {:?}", gateway_registration_input.gateway_name, gateway_registration_input.env_uid, environment_manager_principal_id));
        
            let gateway_uid = rand::thread_rng().gen_range(0..100);
            ic_cdk::print(format!("Gateway UID: {:?}", gateway_uid));

            environment_info.env_gateways.insert(
                gateway_uid,
                GatewayInfo {
                    gateway_name: gateway_registration_input.gateway_name.clone(),
                    gateway_uid,
                    devices: BTreeMap::new(),
                }
            );

            ic_cdk::print(format!("Updated environment: {:?}", environment_info));

            ENVIRONMENT_STORE.with(|environment_store| {
                environment_store.borrow_mut().insert(
                    gateway_registration_input.env_uid,
                    environment_info
                )
            });

            let gateway_registration_result = GatewayRegistrationResult {
                gateway_name: gateway_registration_input.gateway_name,
                gateway_uid,
            };

            ManualReply::one(gateway_registration_result)
        },
        None => panic!("Environment does not exist"),
    }
}



#[ic_cdk_macros::update(name = "registerDeviceInEnvironment", manual_reply = true)]
fn register_device_in_environment(
    environment_manager_principal_id: PrincipalId,
    device_registration_input: DeviceRegistrationInput
) -> ManualReply<DeviceRegistrationResult> {

    match get_environment_info_by_uid(&device_registration_input.env_uid) {
        Some(mut environment_info) => {

            match environment_info.env_gateways.remove(&device_registration_input.gateway_uid) {
                Some(mut gateway_info) => {

                    let device_uid = rand::thread_rng().gen_range(0..100);
                    ic_cdk::print(format!("Device UID: {:?}", device_uid));

                    gateway_info.devices.insert(
                        device_uid,
                        DeviceInfo {
                            device_name: device_registration_input.device_name.clone(),
                        }
                    );

                    environment_info.env_gateways.insert(
                        device_registration_input.gateway_uid,
                        gateway_info
                    );

                    ic_cdk::print(format!("Updated environment: {:?} managed by: {:?}", environment_info, environment_manager_principal_id));

                    ENVIRONMENT_STORE.with(|environment_store| {
                        environment_store.borrow_mut().insert(
                            device_registration_input.env_uid,
                            environment_info
                        )
                    });

                    let device_registration_result = DeviceRegistrationResult {
                        device_name: device_registration_input.device_name,
                        device_uid,
                    };

                    ManualReply::one(device_registration_result)
                },
                None => panic!("Gateway does not exist in environment"),
            }
        },
        None => panic!("Environment does not exist"),
    }
}



#[ic_cdk_macros::update(name = "setUserInEnvironment", manual_reply = true)]
fn set_user_in_environment(user_principal_id: PrincipalId, env_uid: EnvironmentUID) -> ManualReply<EnvironmentInfo> {

    let user_principal = Principal::from_text(user_principal_id).unwrap();

    match get_user_profile_if_exists(user_principal) {
        Some(user_profile) => {
            match get_environment_info_by_uid(&env_uid) {
                Some(environment_info) => {
                    let updated_user_profile = UserProfile {
                        environment_uid : Some(env_uid.to_owned()),
                        ..user_profile
                    };

                    USER_PROFILE_STORE.with(|profile_store| {
                        profile_store.borrow_mut().insert(user_principal, updated_user_profile);
                    });

                    ic_cdk::print(format!("User: {:?} set in environment: {:?}", user_principal, environment_info));

                    ManualReply::one(environment_info)
                },
                None => panic!("Environment does not exist"),
            }
        },
        None => panic!("User does not have a profile")
    }
}



#[ic_cdk_macros::update(name = "resetUserFromEnvironment", manual_reply = true)]
fn reset_user_from_environment(user_principal_id: PrincipalId) -> ManualReply<EnvironmentInfo> {

    let user_principal = Principal::from_text(user_principal_id).unwrap();

    match get_user_profile_if_exists(user_principal) {
        Some(user_profile) => {
            let updated_user_profile = UserProfile {
                environment_uid : None,
                ..user_profile
            };

            USER_PROFILE_STORE.with(|profile_store| {
                profile_store.borrow_mut().insert(user_principal, updated_user_profile)
            });

            match user_profile.environment_uid {
                Some(old_environment_uid) => {
                    match get_environment_info_by_uid(&old_environment_uid) {
                        Some(environment_info) => ManualReply::one(environment_info),
                        None => panic!("Environment does not exist"),
                    }
                },
                None => panic!("User is not in environment"),
            }
        },
        None => panic!("User does not have a profile")
    }
}



#[ic_cdk_macros::update(name = "getUserProfile", manual_reply = true)]
fn get_user_profile(user_principal_id: PrincipalId) -> ManualReply<UserProfile> {

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



fn custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    // TODO get some randomness
    return Ok(());
}



fn get_environment_info_by_uid(environment_uid: &EnvironmentUID) -> Option<EnvironmentInfo> {
    ENVIRONMENT_STORE.with(|environment_store| {
        match environment_store.borrow().get(environment_uid) {
            Some(mut environment_info) => Some(environment_info.to_owned()),
            None => None,
        }
    })
}



fn get_user_profile_if_exists(user_principal: Principal) -> Option<UserProfile> {
    USER_PROFILE_STORE.with(|profile_store| {
        match profile_store.borrow().get(&user_principal) {
            Some(user_profile) => Some(user_profile.to_owned()),
            None => None
        }
    })
}