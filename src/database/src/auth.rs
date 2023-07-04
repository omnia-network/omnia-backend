use crate::STATE;
use candid::candid_method;
use ic_cdk::print;
use ic_cdk_macros::update;
use omnia_types::http::{IpChallengeIndex, IpChallengeNonce, IpChallengeValue};

#[update]
#[candid_method(update)]
async fn init_nonce_to_ip(nonce: IpChallengeNonce, ip_challenge_value: IpChallengeValue) {
    print(format!(
        "Initialized requester info: {:?} for nonce: {:?} ",
        ip_challenge_value, nonce
    ));

    let ip_challenge_index = IpChallengeIndex { nonce };

    STATE.with(|state| {
        state
            .borrow_mut()
            .ip_challenges
            .create(ip_challenge_index.clone(), ip_challenge_value)
            .expect("should create ip challenge");
    });
}
