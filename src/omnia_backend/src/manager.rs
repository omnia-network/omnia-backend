use candid::candid_method;
use ic_cdk::{
    api::{call::call, caller},
    print, trap,
};
use ic_cdk_macros::{query, update};
use ic_ledger_types::{BlockIndex, Operation, Tokens};
use ic_oxigraph::model::{vocab, GraphName, Literal, NamedNode, Quad};
use omnia_core_sdk::access_key::{AccessKeyUID, UniqueAccessKey, ACCESS_KEY_PRICE};
use omnia_types::{
    access_key::{
        AccessKeyCreationArgs, AccessKeyCreationResult, RejectedAccessKey, RejectedAccessKeyReason,
        SignedRequest,
    },
    device::{DeviceAffordances, RegisteredDeviceResult, RegisteredDevicesUidsResult},
    environment::{EnvironmentCreationInput, EnvironmentCreationResult, EnvironmentUID},
    errors::GenericResult,
    gateway::{
        GatewayPrincipalId, GatewayRegistrationInput, InitializedGatewayValue,
        MultipleRegisteredGatewayResult, RegisteredGatewayResult,
    },
    http::IpChallengeNonce,
    updates::{PairingPayload, UpdateValueOption, UpdateValueResult},
    virtual_persona::VirtualPersonaPrincipalId,
};
use omnia_utils::ic::{get_transaction_hash, principal_to_account};

use crate::{
    rdf::{BotNode, HttpNode, OmniaNode, SarefNode, TdNode, UrnNode},
    utils::{
        get_backend_principal, get_database_principal, is_valid_signature, query_ledger_block,
    },
    RDF_DB,
};

#[update(name = "createEnvironment")]
#[candid_method(update, rename = "createEnvironment")]
async fn create_environment(
    environment_creation_input: EnvironmentCreationInput,
) -> GenericResult<EnvironmentCreationResult> {
    let environment_manager_principal_id = caller().to_string();

    let (virtual_persona_exists,): (bool,) = call(
        get_database_principal(),
        "check_if_virtual_persona_exists",
        (environment_manager_principal_id.clone(),),
    )
    .await
    .unwrap();
    match virtual_persona_exists {
        true => {
            let (environment_creation_result,): (GenericResult<EnvironmentCreationResult>,) = call(
                get_database_principal(),
                "create_new_environment",
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

            // register the environment in the RDF database
            match environment_creation_result {
                Ok(result) => {
                    let quad = Quad::new(
                        UrnNode::new_uuid(&result.env_uid),
                        vocab::rdf::TYPE,
                        BotNode::from("Zone"),
                        GraphName::DefaultGraph,
                    );

                    RDF_DB.with(|rdf_db| match rdf_db.borrow().insert(&quad) {
                        Ok(_) => Ok(result),
                        Err(err) => Err(format!("Error inserting quad: {:?}", err)),
                    })
                }
                Err(err) => Err(err),
            }
        }
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

    let is_registered = call::<(GatewayPrincipalId,), (bool,)>(
        get_database_principal(),
        "is_gateway_registered",
        (gateway_principal_id.clone(),),
    )
    .await
    .unwrap()
    .0;

    if !is_registered {
        print(format!(
            "Gateway with principal ID: {:?} is not yet registered",
            gateway_principal_id
        ));
        let principal_id =
            call::<(IpChallengeNonce, GatewayPrincipalId), (GenericResult<GatewayPrincipalId>,)>(
                get_database_principal(),
                "init_gateway_by_ip",
                (nonce, gateway_principal_id),
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
async fn get_initialized_gateways(
    nonce: IpChallengeNonce,
) -> GenericResult<Vec<InitializedGatewayValue>> {
    let initialized_gateway_principals_result: GenericResult<Vec<InitializedGatewayValue>> =
        match call(
            get_database_principal(),
            "get_initialized_gateways_by_ip",
            (nonce,),
        )
        .await
        .unwrap()
        {
            (Ok(initialized_gateway_principals),) => {
                print(format!(
                    "Initialized gateways in the local network have principals {:?}",
                    initialized_gateway_principals
                ));
                Ok(initialized_gateway_principals)
            }
            (Err(e),) => Err(e),
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
        "register_gateway_in_environment",
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
async fn get_registered_gateways(
    environment_uid: EnvironmentUID,
) -> MultipleRegisteredGatewayResult {
    let (res,): (MultipleRegisteredGatewayResult,) = call(
        get_database_principal(),
        "get_registered_gateways_in_environment",
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
        "get_gateway_updates_by_principal",
        (gateway_principal_id,),
    )
    .await
    .unwrap()
    .0
}

#[update(name = "pairNewDevice")]
#[candid_method(update, rename = "pairNewDevice")]
async fn pair_new_device(
    nonce: IpChallengeNonce,
    gateway_principal_id: GatewayPrincipalId,
    pairing_payload: PairingPayload,
) -> UpdateValueResult {
    let manager_principal_id = caller().to_string();

    call::<
        (
            IpChallengeNonce,
            VirtualPersonaPrincipalId,
            GatewayPrincipalId,
            PairingPayload,
        ),
        (UpdateValueResult,),
    >(
        get_database_principal(),
        "pair_new_device_on_gateway",
        (
            nonce,
            manager_principal_id,
            gateway_principal_id,
            pairing_payload,
        ),
    )
    .await
    .unwrap()
    .0
}

#[update(name = "registerDevice")]
#[candid_method(update, rename = "registerDevice")]
async fn register_device(
    nonce: IpChallengeNonce,
    affordances: DeviceAffordances,
) -> RegisteredDeviceResult {
    let gateway_principal_id = caller().to_string();

    let registered_device =
        call::<(IpChallengeNonce, GatewayPrincipalId), (RegisteredDeviceResult,)>(
            get_database_principal(),
            "register_device_on_gateway",
            (nonce, gateway_principal_id),
        )
        .await
        .unwrap()
        .0?;

    let device_url = registered_device.clone().1.device_url;

    let device_node = NamedNode::new(device_url.clone()).map_err(|err| {
        format!(
            "Error while creating device node for device with URL: {:?} {:?}",
            device_url, err
        )
    })?;

    let mut quads: Vec<Quad> = vec![
        // device declaration
        Quad::new(
            device_node.clone(),
            vocab::rdf::TYPE,
            SarefNode::from("Device"),
            GraphName::DefaultGraph,
        ),
        // device - environment relation
        Quad::new(
            UrnNode::new_uuid(&registered_device.1.env_uid),
            BotNode::from("hasElement"),
            device_node.clone(),
            GraphName::DefaultGraph,
        ),
    ];

    // device required HTTP headers
    // TODO: define better names for HTTP headers
    let required_headers = registered_device.clone().1.required_headers;
    if required_headers.is_some() {
        required_headers.unwrap().iter().enumerate().for_each(
            |(i, (header_name, header_value))| {
                let header_node = OmniaNode::from(&format!("HTTPHeader{}", i));

                quads.extend_from_slice(&[
                    Quad::new(
                        header_node.clone(),
                        vocab::rdf::TYPE,
                        HttpNode::from("RequestHeader"),
                        GraphName::DefaultGraph,
                    ),
                    Quad::new(
                        header_node.clone(),
                        HttpNode::from("fieldName"),
                        Literal::new_simple_literal(header_name),
                        GraphName::DefaultGraph,
                    ),
                    Quad::new(
                        header_node.clone(),
                        HttpNode::from("fieldValue"),
                        Literal::new_simple_literal(header_value),
                        GraphName::DefaultGraph,
                    ),
                    Quad::new(
                        device_node.clone(),
                        OmniaNode::from("requiresHeader"),
                        header_node,
                        GraphName::DefaultGraph,
                    ),
                ]);
            },
        );
    }

    quads.extend(affordances.properties.iter().map(|affordance| {
        Quad::new(
            device_node.clone(),
            TdNode::from("hasPropertyAffordance"),
            SarefNode::from_prefixed(affordance),
            GraphName::DefaultGraph,
        )
    }));

    quads.extend(affordances.actions.iter().map(|affordance| {
        Quad::new(
            device_node.clone(),
            TdNode::from("hasActionAffordance"),
            SarefNode::from_prefixed(affordance),
            GraphName::DefaultGraph,
        )
    }));

    // TODO: handle outcall errors. For example we may want to retry or remove the registered device
    quads.iter().for_each(|quad| {
        RDF_DB.with(|rdf_db| match rdf_db.borrow().insert(quad) {
            Ok(_) => (),
            Err(e) => {
                trap(&format!("Error inserting quad: {}", e));
            }
        });
    });

    Ok(registered_device)
}

#[update(name = "getRegisteredDevices")]
#[candid_method(update, rename = "getRegisteredDevices")]
async fn get_registered_devices() -> RegisteredDevicesUidsResult {
    let gateway_principal_id = caller().to_string();

    call::<(GatewayPrincipalId,), (RegisteredDevicesUidsResult,)>(
        get_database_principal(),
        "get_registered_devices_on_gateway",
        (gateway_principal_id,),
    )
    .await
    .unwrap()
    .0
}

#[update(name = "obtainAccessKey")]
#[candid_method(update, rename = "obtainAccessKey")]
async fn obtain_access_key(block_index: BlockIndex) -> GenericResult<AccessKeyUID> {
    let caller_principal = caller();

    let ledger_block = query_ledger_block(block_index).await?;

    if let Some(block) = ledger_block {
        print(format!("Block at index {:?}: {:?}", block_index, block));

        if let Some(Operation::Transfer {
            from, to, amount, ..
        }) = block.transaction.operation
        {
            let caller_account = principal_to_account(caller_principal);
            let backend_account = principal_to_account(get_backend_principal());

            // check if the caller of this method is the same principal that paid for the access key
            if from != caller_account {
                return Err(String::from("Caller account does not match the sender"));
            }
            // check if the receiver of the transfer was the Omnia Backend canister
            if to != backend_account {
                return Err(String::from(
                    "Receiver does not match the Omnia Backend account",
                ));
            }
            // check if the amount of the transfer is correct
            if amount != ACCESS_KEY_PRICE {
                return Err(String::from(
                    "Transferred amount does not match the price of the access key",
                ));
            }

            let access_key_value = call::<(AccessKeyCreationArgs,), (AccessKeyCreationResult,)>(
                get_database_principal(),
                "create_new_access_key",
                (AccessKeyCreationArgs {
                    owner: caller_principal,
                    transaction_hash: get_transaction_hash(block.transaction),
                },),
            )
            .await
            .unwrap()
            .0?;

            print(format!("Access key value: {:?}", access_key_value));

            return Ok(access_key_value.get_key());
        }

        return Err(String::from("Block does not contain a transfer operation"));
    }
    Err(String::from("No block found"))
}

#[update(name = "reportSignedRequests")]
#[candid_method(update, rename = "reportSignedRequests")]
async fn report_signed_requests(
    signed_requests: Vec<SignedRequest>,
) -> GenericResult<Vec<RejectedAccessKey>> {
    print(format!(
        "Reporting {} signed requests...",
        signed_requests.len()
    ));

    let mut unique_access_keys_to_spend: Vec<UniqueAccessKey> = vec![];
    let mut rejected_access_keys: Vec<RejectedAccessKey> = vec![];

    // check if the signature of the signed request is valid
    for signed_request in signed_requests {
        match is_valid_signature(
            signed_request.get_signature(),
            signed_request.get_unique_access_key().serialize(),
            signed_request.get_requester_principal_id(),
        )
        .await
        {
            Ok(true) => {
                unique_access_keys_to_spend.push(signed_request.get_unique_access_key());
            }
            Ok(false) => {
                rejected_access_keys.push(RejectedAccessKey {
                    key: signed_request.get_unique_access_key().get_key(),
                    reason: RejectedAccessKeyReason::InvalidSignature,
                });
            }
            Err(e) => {
                rejected_access_keys.push(RejectedAccessKey {
                    key: signed_request.get_unique_access_key().get_key(),
                    reason: RejectedAccessKeyReason::SignatureVerificationError(e),
                });
            }
        }
    }

    // spend the unique access keys and get the rejected ones
    let rejected_keys = call::<(Vec<UniqueAccessKey>,), (GenericResult<Vec<RejectedAccessKey>>,)>(
        get_database_principal(),
        "spend_requests_for_keys",
        (unique_access_keys_to_spend,),
    )
    .await
    .unwrap()
    .0?;

    rejected_access_keys.extend(rejected_keys);

    print(format!(
        "{} signed requests were rejected!",
        rejected_access_keys.len()
    ));

    Ok(rejected_access_keys)
}

#[query(name = "getAccessKeyPrice")]
#[candid_method(query, rename = "getAccessKeyPrice")]
fn get_access_key_price() -> Tokens {
    ACCESS_KEY_PRICE
}

#[update(name = "getAccessKeyPriceAsUpdate")]
#[candid_method(update, rename = "getAccessKeyPriceAsUpdate")]
/// Needed for inter-canister calls
fn get_access_key_price_as_update() -> Tokens {
    ACCESS_KEY_PRICE
}
