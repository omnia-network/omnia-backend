use std::collections::BTreeSet;
use std::cell::RefCell;
use getrandom::register_custom_getrandom;

register_custom_getrandom!(custom_getrandom);

fn custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    // TODO get some randomness
    return Ok(());
}

type GatewayUID = String;

type InitializedGatewayStore = BTreeSet<GatewayUID>;

thread_local! {
    static INITIALIZED_GATEWAY_STORE: RefCell<InitializedGatewayStore> = RefCell::default();
}

mod user;
mod manager;