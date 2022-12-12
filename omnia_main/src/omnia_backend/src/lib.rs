use ic_cdk::api;

#[ic_cdk_macros::import(canister = "database")]
struct Database;

#[ic_cdk_macros::update(name = "getProfile")]
async fn get_profile() -> Box<UserProfile> {
    let user_principal = api::caller();

    let user_profile = Database::getUserProfile(user_principal.to_string()).await.0;

    ic_cdk::print(format!("Created user profile: {:?}", user_profile));

    user_profile
}

#[ic_cdk_macros::update(name = "setUserInEnvironment")]
fn set_user_in_environment(env_uid: String) -> Box<EnvironmentInfo> {
    // TODO: add user to environment
    let environment_info = Box::new(EnvironmentInfo {
        env_name: String::from("Example environment"),
        env_uid,
    });

    ic_cdk::print(format!("Setting user in environment: {:?}", environment_info));

    environment_info
}

#[ic_cdk_macros::update(name = "registerEnvironment")]
async fn register_environment(
    environment_registration_input: EnvironmentRegistrationInput
) -> Box<EnvironmentRegistrationResult> {

    let environment_manager_principal = api::caller();

    let environment_registration_result = Database::initializeNewEnvironment(
        environment_manager_principal.to_string(),
        Box::new(environment_registration_input)
    ).await.0;

    ic_cdk::print(format!("Initialized environment: {:?}", environment_registration_result));

    environment_registration_result
}