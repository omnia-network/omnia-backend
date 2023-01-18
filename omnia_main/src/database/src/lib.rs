use getrandom::register_custom_getrandom;
use ic_cdk::api::call::ManualReply;
use rand::Rng;
use hex;

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

fn generate_local_uuid() -> String {
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

#[ic_cdk_macros::update(name = "generateUuid", manual_reply = true)]
fn generate_uuid() -> ManualReply<String> {
    let uuid = generate_local_uuid();
    ManualReply::one(uuid)
}

mod environment;
mod profile;
