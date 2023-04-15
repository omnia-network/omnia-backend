use candid::candid_method;
use ic_cdk::{
    api::{call::call, caller},
    print,
};
use ic_cdk_macros::update;
use omnia_types::{
    environment::{EnvironmentCreationInput, EnvironmentCreationResult, EnvironmentUID},
    gateway::{RegisteredGatewayResult, GatewayRegistrationInput, MultipleRegisteredGatewayResult, InitializedGatewayValue, GatewayPrincipalId}, http::IpChallengeNonce, errors::{GenericResult, GenericError}, updates::{UpdateValueResult, UpdateValueOption}, virtual_persona::VirtualPersonaPrincipalId
};

use crate::utils::get_database_principal;

#[update(name = "createEnvironment")]
#[candid_method(update, rename = "createEnvironment")]
async fn create_environment(
    environment_creation_input: EnvironmentCreationInput,
) -> GenericResult<EnvironmentCreationResult> {
    let environment_manager_principal_id = caller().to_string();

    let (virtual_persona_exists, ): (bool, ) = call(
        get_database_principal(),
        "checkIfVirtualPersonaExists",
        (environment_manager_principal_id.clone(),),
    ).await.unwrap();
    match virtual_persona_exists {
        true => {
            let (environment_creation_result,): (Result<EnvironmentCreationResult, GenericError>,) = call(
                get_database_principal(),
                "createNewEnvironment",
                (
                    environment_manager_principal_id,
                    Box::new(environment_creation_input),
                ),
            )
            .await
            .unwrap();
        
            print(format!(
                "Created new environment: {:?}",
                environment_creation_result
            ));
        
            environment_creation_result
        },
        false => {
            let err = format!(
                "Virtual persona with principal id: {:?} does not exist",
                environment_manager_principal_id
            );

            println!("{}", err);
            Err(err)
        }
    }
}

#[update(name = "initGateway")]
#[candid_method(update, rename = "initGateway")]
async fn init_gateway(nonce: IpChallengeNonce) -> GenericResult<GatewayPrincipalId> {
    let gateway_principal_id = caller().to_string();

    let is_registered = 
        call::<
            (GatewayPrincipalId, ),
            (bool, )
        >(
            get_database_principal(),
            "isGatewayRegistered",
            (
                gateway_principal_id.clone(),
            )
        )
        .await
        .unwrap()
        .0;

    if !is_registered {
        print(format!("Gateway with principal ID: {:?} is not yet registered", gateway_principal_id));
        let principal_id =
            call::<
                (IpChallengeNonce, GatewayPrincipalId),
                (GenericResult<GatewayPrincipalId>,)
            >(
                get_database_principal(),
                "initGatewayByIp", 
                (
                    nonce,
                    gateway_principal_id,
                )
            )
            .await
            .unwrap()
            .0?;

        return Ok(principal_id);
    }
    Ok(gateway_principal_id)
}

#[update(name = "getInitializedGateways")]
#[candid_method(update, rename = "getInitializedGateways")]
async fn get_initialized_gateways(nonce: IpChallengeNonce) -> GenericResult<Vec<InitializedGatewayValue>> {
    
    let initialized_gateway_principals_result: GenericResult<Vec<InitializedGatewayValue>> = match call(get_database_principal(), "getInitializedGatewaysByIp", (nonce, ))
        .await
        .unwrap()
    {
        (Ok(initialized_gateway_principals),) => {
            print(format!("Initialized gateways in the local network have principals {:?}", initialized_gateway_principals));
            Ok(initialized_gateway_principals)
        },
        (Err(e),) => Err(e)
    };
    initialized_gateway_principals_result
}

#[update(name = "registerGateway")]
#[candid_method(update, rename = "registerGateway")]
async fn register_gateway(
    nonce: IpChallengeNonce,
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

#[update(name = "getGatewayUpdates")]
#[candid_method(update, rename = "getGatewayUpdates")]
async fn get_gateway_updates() -> UpdateValueOption {
    let gateway_principal_id = caller().to_string();

    call::<(GatewayPrincipalId,), (UpdateValueOption,)>(
        get_database_principal(),
        "getGatewayUpdatesByPrincipal",
        (gateway_principal_id,),
    )
    .await
    .unwrap()
    .0
}

#[update(name = "pairNewDevice")]
#[candid_method(update, rename = "pairNewDevice")]
async fn pair_new_device(nonce: IpChallengeNonce, gateway_principal_id: GatewayPrincipalId) -> UpdateValueResult {
    let manager_principal_id = caller().to_string();

    call::<(IpChallengeNonce, VirtualPersonaPrincipalId, GatewayPrincipalId,), (UpdateValueResult,)>(
        get_database_principal(),
        "pairNewDeviceOnGateway",
        (
            nonce,
            manager_principal_id,
            gateway_principal_id,
        ),
    )
    .await
    .unwrap()
    .0
}

