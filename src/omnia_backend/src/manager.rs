use candid::{candid_method, Principal};
use ic_cdk::{
    api::{
        call::{call, call_with_payment},
        caller,
    },
    print, trap,
};
use ic_cdk_macros::{query, update};
use ic_ledger_types::{BlockIndex, Operation, Tokens};
use ic_oxigraph::model::{vocab, GraphName, Literal, NamedNode, Quad};
use omnia_types::{
    device::{DeviceAffordances, RegisteredDeviceResult, RegisteredDevicesUidsResult},
    environment::{EnvironmentCreationInput, EnvironmentCreationResult, EnvironmentUID},
    errors::{GenericError, GenericResult},
    gateway::{
        GatewayPrincipalId, GatewayRegistrationInput, InitializedGatewayValue,
        MultipleRegisteredGatewayResult, RegisteredGatewayResult,
    },
    http::IpChallengeNonce,
    request_key::{RequestKeyCreationResult, RequestKeyUID},
    signature::{
        ECDSAPublicKey, ECDSAPublicKeyReply, EcdsaKeyIds, PublicKeyReply, SignWithECDSA,
        SignWithECDSAReply, SignatureReply, SignatureVerificationReply,
    },
    updates::{PairingPayload, UpdateValueOption, UpdateValueResult},
    virtual_persona::VirtualPersonaPrincipalId,
};

use crate::{
    rdf::{BotNode, HttpNode, OmniaNode, SarefNode, TdNode, UrnNode},
    utils::{
        check_balance, get_backend_principal, get_database_principal, mgmt_canister_id,
        principal_to_account, query_one_block, sha256, transfer_to,
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
        "checkIfVirtualPersonaExists",
        (environment_manager_principal_id.clone(),),
    )
    .await
    .unwrap();
    match virtual_persona_exists {
        true => {
            let (environment_creation_result,): (Result<EnvironmentCreationResult, GenericError>,) =
                call(
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
        "isGatewayRegistered",
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
                "initGatewayByIp",
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
            "getInitializedGatewaysByIp",
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
async fn get_registered_gateways(
    environment_uid: EnvironmentUID,
) -> MultipleRegisteredGatewayResult {
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
        "pairNewDeviceOnGateway",
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
            "registerDeviceOnGateway",
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
        "getRegisteredDevicesOnGateway",
        (gateway_principal_id,),
    )
    .await
    .unwrap()
    .0
}

#[update(name = "transferIcpsToPrincipal")]
#[candid_method(update, rename = "transferIcpsToPrincipal")]
async fn transfer_icps_to_principal(principal_id: String, amount: Tokens) -> BlockIndex {
    check_balance(Principal::from_text(principal_id.clone()).expect("valid principal")).await;

    let block_index = transfer_to(
        Principal::from_text(principal_id.clone()).expect("valid principal"),
        amount,
    )
    .await;

    check_balance(Principal::from_text(principal_id.clone()).expect("valid principal")).await;

    block_index
}

#[update(name = "getRequestKey")]
#[candid_method(update, rename = "getRequestKey")]
async fn get_request_key(block_index: BlockIndex) -> GenericResult<RequestKeyUID> {
    let caller_principal = caller();
    let caller_account = principal_to_account(caller_principal);

    let block_opt = query_one_block(block_index).await?;

    if let Some(block) = block_opt {
        print(format!("Block at index {:?}: {:?}", block_index, block));
        if let Some(Operation::Transfer {
            from, to, amount, ..
        }) = block.transaction.operation
        {
            let backend_account = principal_to_account(get_backend_principal());
            let request_key_price = Tokens::from_e8s(1000000);

            // TODO: check if this transfer hasn't been used to pay for a request key yet

            // check if the caller of this method is the same principal that paid for the request key
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
            if amount != request_key_price {
                return Err(String::from(
                    "Transferred amount does not match the price of the request key",
                ));
            }

            let request_key_creation_result = call::<(String,), (RequestKeyCreationResult,)>(
                get_database_principal(),
                "createNewRequestKey",
                (caller_principal.to_string(),),
            )
            .await
            .unwrap()
            .0;

            print(format!(
                "Request key creation result: {:?}",
                request_key_creation_result
            ));

            return Ok(request_key_creation_result.unwrap().key);
        }
    }
    Err(String::from("No block found"))
}

#[update(name = "signMessage")]
#[candid_method(update, rename = "signMessage")]
async fn sign_message(message: String) -> Result<SignatureReply, String> {
    let request = SignWithECDSA {
        message_hash: sha256(&message).to_vec(),
        derivation_path: vec![],
        key_id: EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id(),
    };

    let (response,): (SignWithECDSAReply,) = call_with_payment(
        mgmt_canister_id(),
        "sign_with_ecdsa",
        (request,),
        25_000_000_000,
    )
    .await
    .map_err(|e| format!("sign_with_ecdsa failed {}", e.1))?;

    Ok(SignatureReply {
        signature_hex: hex::encode(&response.signature),
    })
}

#[query(name = "verifyMessage")]
#[candid_method(query, rename = "verifyMessage")]
async fn verify_message(
    signature_hex: String,
    message: String,
    public_key_hex: String,
) -> Result<SignatureVerificationReply, String> {
    let signature_bytes = hex::decode(&signature_hex).expect("failed to hex-decode signature");
    let pubkey_bytes = hex::decode(&public_key_hex).expect("failed to hex-decode public key");
    let message_bytes = message.as_bytes();

    use k256::ecdsa::signature::Verifier;
    let signature = k256::ecdsa::Signature::try_from(signature_bytes.as_slice())
        .expect("failed to deserialize signature");
    let is_signature_valid = k256::ecdsa::VerifyingKey::from_sec1_bytes(&pubkey_bytes)
        .expect("failed to deserialize sec1 encoding into public key")
        .verify(message_bytes, &signature)
        .is_ok();

    Ok(SignatureVerificationReply { is_signature_valid })
}

#[update(name = "getCanisterPublicKey")]
#[candid_method(update, rename = "getCanisterPublicKey")]
async fn get_canister_public_key(canister_id: String) -> Result<PublicKeyReply, String> {
    let request = ECDSAPublicKey {
        canister_id: Principal::from_text(canister_id).expect("valid principal"),
        derivation_path: vec![],
        key_id: EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id(),
    };

    let (res,): (ECDSAPublicKeyReply,) = call(mgmt_canister_id(), "ecdsa_public_key", (request,))
        .await
        .map_err(|e| format!("ecdsa_public_key failed {}", e.1))?;

    Ok(PublicKeyReply {
        public_key_hex: hex::encode(&res.public_key),
    })
}

#[query(name = "whoAmI")]
#[candid_method(query, rename = "whoAmI")]
fn who_am_i() -> String {
    caller().to_string()
}
