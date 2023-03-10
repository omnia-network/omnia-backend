use candid::candid_method;
use ic_cdk::{
    api::{call::call, caller},
    print,
};
use ic_cdk_macros::update;
use omnia_types::{
    environment::{EnvironmentCreationInput, EnvironmentCreationResult, EnvironmentUID},
    gateway::{RegisteredGatewayResult, GatewayRegistrationInput, MultipleRegisteredGatewayResult, GatewayUID}, http::CanisterCallNonce
};

use crate::utils::get_database_principal;

#[update(name = "createEnvironment")]
#[candid_method(update, rename = "createEnvironment")]
async fn create_environment(
    environment_creation_input: EnvironmentCreationInput,
) -> Result<EnvironmentCreationResult, ()> {
    let environment_manager_principal = caller();

    let (virtual_persona_exists, ): (bool, ) = call(
        get_database_principal(),
        "checkIfVirtualPersonaExists",
        (environment_manager_principal,),
    ).await.unwrap();
    match virtual_persona_exists {
        true => {
            let (environment_creation_result,): (EnvironmentCreationResult,) = call(
                get_database_principal(),
                "createNewEnvironment",
                (
                    environment_manager_principal.to_string(),
                    Box::new(environment_creation_input),
                ),
            )
            .await
            .unwrap();
        
            print(format!(
                "Created new environment: {:?}",
                environment_creation_result
            ));
        
            Ok(environment_creation_result)
        },
        false => {
            Err(())
        }
    }
}

#[update(name = "initGateway")]
#[candid_method(update, rename = "initGateway")]
async fn init_gateway(nonce: CanisterCallNonce) -> Result<String, ()> {
    let gateway_principal = caller();

    match call(get_database_principal(), "initGatewayWithIp", (
        nonce,
        gateway_principal.to_string(),
    ))
    .await
    .unwrap() {
        (Ok(gateway_uuid),) => {
            print(format!("Initialized gateway with UUID: {:?}", gateway_uuid));
            Ok(gateway_uuid)
        },
        (Err(()),) => Err(())
    }
}

#[update(name = "getInitializedGateways")]
#[candid_method(update, rename = "getInitializedGateways")]
async fn get_initialized_gateways(nonce: CanisterCallNonce) -> Result<Vec<GatewayUID>, ()> {
    
    let initialized_gateway_principals_result: Result<Vec<GatewayUID>, ()> = match call(get_database_principal(), "getInitializedGatewaysByIp", (nonce, ))
        .await
        .unwrap()
    {
        (Ok(initialized_gateway_principals),) => {
            print(format!("Initialized gateways in the local network have principals {:?}", initialized_gateway_principals));
            Ok(initialized_gateway_principals)
        },
        (Err(()),) => Err(())
    };
    initialized_gateway_principals_result
}

#[update(name = "registerGateway")]
#[candid_method(update, rename = "registerGateway")]
async fn register_gateway(
    nonce: CanisterCallNonce,
    gateway_registration_input: GatewayRegistrationInput,
) -> RegisteredGatewayResult {
    let environment_manager_principal = caller();

    let (gateway_registration_result,): (RegisteredGatewayResult,) = call(
        get_database_principal(),
        "registerGatewayInEnvironment",
        (
            nonce,
            environment_manager_principal.to_string(),
            Box::new(gateway_registration_input),
        ),
    )
    .await
    .unwrap();

    gateway_registration_result
}

#[update(name = "getRegisteredGateways")]
#[candid_method(update, rename = "getRegisteredGateways")]
async fn get_registered_gateways(environment_uid: EnvironmentUID) -> MultipleRegisteredGatewayResult {
    let (res,): (MultipleRegisteredGatewayResult,) = call(
        get_database_principal(),
        "getRegisteredGatewaysInEnvironment",
        (environment_uid.clone(),),
    )
    .await
    .unwrap();

    res
}