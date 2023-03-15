// use std::collections::BTreeMap;
// use candid::candid_method;
// use ic_cdk::print;
// use ic_cdk_macros::update;
// use omnia_types::{
//     environment::{EnvironmentCreationInput, EnvironmentCreationResult, EnvironmentUID, Environment},
//     gateway::{
//         RegisteredGateway, RegisteredGatewayResult, GatewayRegistrationInput,
//         MultipleRegisteredGatewayResult, GatewayPrincipalId,
//     },
//     virtual_persona::VirtualPersonaPrincipalId, http::CanisterCallNonce
// };

// use crate::{uuid::generate_uuid, STATE};

// #[update(name = "initGatewayByIp")]
// #[candid_method(update, rename = "initGatewayByIp")]
// async fn init_gateway_with_ip(nonce: CanisterCallNonce, gateway_principal_id: GatewayPrincipalId) -> Result<String, ()> {

//     STATE.with(|state| {
//         let mut mutable_state = state.borrow_mut();
//         match mutable_state.consume_ip_challenge(&nonce)
//         {
//             Some(gateway_request_info) => {        
//                 mutable_state.initialize_gateway_by_ip(gateway_request_info.requester_ip, gateway_principal_id.clone());
//                 Ok(gateway_principal_id)
    
//             },
//             None => {
//                 Err(())
//             }
//         }
//     })
// }

// #[update(name = "getInitializedGatewaysByIp")]
// #[candid_method(update, rename = "getInitializedGatewaysByIp")]
// async fn get_initialized_gateways_by_ip(nonce: CanisterCallNonce) -> Result<Vec<GatewayPrincipalId>, ()> {
//     STATE.with(|state| {
//         let mut mutable_state = state.borrow_mut();
//         match mutable_state.consume_ip_challenge(&nonce) {
//             Some(virtual_persona_request_info) => mutable_state.get_initialized_gateways_by_ip(&virtual_persona_request_info.requester_ip),
//             None => {
//                 Err(())
//             }
//         }
//     })
// }

// #[update(name = "createNewEnvironment")]
// #[candid_method(update, rename = "createNewEnvironment")]
// async fn create_new_environment(
//     environment_manager_principal_id: VirtualPersonaPrincipalId,
//     environment_creation_input: EnvironmentCreationInput,
// ) -> EnvironmentCreationResult {
//     print(format!(
//         "Creating new environment: {:?} managed by: {:?}",
//         environment_creation_input, environment_manager_principal_id
//     ));

//     let environment_uid = generate_uuid().await;
//     print(format!("Environment UID: {:?}", environment_uid));

//     STATE.with(|state| {
//         let mut mutable_state = state.borrow_mut();
//         mutable_state.create_environment(
//             environment_uid.clone(),
//             Environment {
//                 env_name: environment_creation_input.env_name.clone(),
//                 env_ip: None,
//                 env_users_principals_ids: BTreeMap::default(),
//                 env_gateway_principal_ids: BTreeMap::default(),
//                 env_manager_principal_id: environment_manager_principal_id,
//             },
//         );
//     });

//     let environment_creation_result = EnvironmentCreationResult {
//         env_name: environment_creation_input.env_name,
//         env_uid: environment_uid,
//     };

//     print(format!(
//         "Created new environment: {:?}",
//         environment_creation_result
//     ));

//     environment_creation_result
// }

// #[update(name = "registerGatewayInEnvironment")]
// #[candid_method(update, rename = "registerGatewayInEnvironment")]
// fn register_gateway_in_environment(
//     nonce: CanisterCallNonce,
//     environment_manager_principal_id: VirtualPersonaPrincipalId,
//     gateway_registration_input: GatewayRegistrationInput,
// ) -> RegisteredGatewayResult {

//     STATE.with(|state| {
//         let mut mutable_state = state.borrow_mut();
//         match mutable_state.consume_ip_challenge(&nonce)
//         {
//             Some(virtual_persona_request_info) => {
//                 match mutable_state.consume_initialized_gateway(&virtual_persona_request_info.requester_ip) {
//                     Some(gateway_principal_id) => {
//                         // register mapping IP to Environment UID in order to be able to retrive the UID of the environment from the IP when a User registers in an environment
//                         mutable_state.create_ip_to_uid_environment_mapping(virtual_persona_request_info.requester_ip.clone(), gateway_registration_input.env_uid.clone());

//                         let registered_gateway = RegisteredGateway {
//                             gateway_name: gateway_registration_input.gateway_name,
//                             gateway_ip: virtual_persona_request_info.requester_ip,
//                             env_uid: gateway_registration_input.env_uid.clone(),
//                         };

//                         mutable_state.create_registered_gateway(gateway_principal_id.clone(), registered_gateway.clone());

//                         match mutable_state.get_environment_by_uid(&gateway_registration_input.env_uid) {
//                             Ok(environment) => {
//                                 print(format!(
//                                     "Registering gateway in environment with UID: {:?} managed by: {:?}",
//                                     gateway_registration_input.env_uid,
//                                     environment_manager_principal_id
//                                 ));

//                                 // add principal ID of registered Gateway to Environment
//                                 environment.env_gateway_principal_ids.insert(gateway_principal_id, ());
//                                 print(format!("Updated environment: {:?}", environment));
//                                 Ok(registered_gateway)
//                             },
//                             Err(e) => Err(e),
//                         }
//                     },
//                     None => {
//                         let err = format!(
//                             "Gateway with IP {:?} has not been initialized",
//                             virtual_persona_request_info.requester_ip
//                         );
    
//                         print(err.as_str());
//                         Err(err)
//                     }
//                 }
//             },
//             None => {
//                 let err = format!(
//                     "Did not receive http request with nonce {:?} before canister call",
//                     nonce
//                 );
    
//                 print(err.as_str());
//                 Err(err)
//             },
//         }
//     })
// }

// #[update(name = "getRegisteredGatewaysInEnvironment")]
// #[candid_method(update, rename = "getRegisteredGatewaysInEnvironment")]
// fn get_registered_gateways_in_environment(environment_uid: EnvironmentUID) -> MultipleRegisteredGatewayResult {
//     STATE.with(|state| {
//         let mut mutable_state = state.borrow_mut();
//         match mutable_state.get_environment_by_uid(&environment_uid) {
//             Ok(environment) => {
//                 let gateway_principal_ids: Vec<GatewayPrincipalId> = environment
//                     .env_gateway_principal_ids
//                     .iter()
//                     .fold(vec![], |mut gateway_principal_ids, (gateway_principal_id, _)| 
//                 {
//                     gateway_principal_ids.push(gateway_principal_id.clone());
//                     gateway_principal_ids
//                 });
//                 let mut registered_gateways: Vec<RegisteredGateway> = vec![];
//                 for gateway_principal_id in gateway_principal_ids {
//                     match mutable_state.get_registered_gateway_by_principal_id(&gateway_principal_id) {
//                         Some(registered_gateway) => registered_gateways.push(registered_gateway.clone()),
//                         None => ()
//                     };
//                 }
//                 print(format!("{:?}", registered_gateways));
//                 Ok(registered_gateways)
//             }
//             Err(e) => Err(e) 
//         }
//     })

// }