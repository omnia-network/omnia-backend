use rand::Rng;
use std::collections::BTreeSet;
use std::cell::RefCell;
use getrandom::register_custom_getrandom;

register_custom_getrandom!(custom_getrandom);

fn custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    // TODO get some randomness
    return Ok(());
}

fn create_byte_vector() -> Vec<u8> {
    let mut random_bytes = rand::thread_rng().gen::<[u8; 16]>();
    random_bytes[6] = (random_bytes[6] & 0x0f) | 0x40;
    random_bytes[8] = (random_bytes[8] & 0x3f) | 0x80;
    random_bytes.to_vec()
}

fn hex_encode(byte_vector: Vec<u8>) -> Vec<String> {
    byte_vector
        .into_iter()
        .map(|byte| hex::encode([byte]))
        .collect()
}

fn generate_uuid() -> String {
    let hex_vector = hex_encode(create_byte_vector());
    let mut uuid = String::new();
    for (index, byte) in hex_vector.iter().enumerate() {
        uuid.push_str(byte);
        if [3, 5, 7, 9].contains(&index) {
            uuid.push('-');
        }
    }
    uuid
}

type GatewayUID = String;

type InitializedGatewayStore = BTreeSet<GatewayUID>;

thread_local! {
    static INITIALIZED_GATEWAY_STORE: RefCell<InitializedGatewayStore> = RefCell::default();
}

mod user;
mod manager;