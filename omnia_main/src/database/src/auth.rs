// use candid::{candid_method};
// use ic_cdk::print;
// use ic_cdk_macros::update;
// use omnia_types::http::{CanisterCallNonce, RequesterInfo};
// use crate::VIRTUAL_PERSONAS_STATE;

// #[update(name = "initNonceToIp")]
// #[candid_method(update, rename = "initNonceToIp")]
// async fn init_nonce_to_ip(nonce: CanisterCallNonce, requester_info: RequesterInfo) -> Option<RequesterInfo> {

//     print(format!("Initialized requester info: {:?} for nonce: {:?} ", requester_info, nonce));

//     STATE.with(|state| {
//         state
//             .borrow_mut()
//             .initialized_nonce_to_ip
//             .insert(nonce, requester_info)
//     })
// }