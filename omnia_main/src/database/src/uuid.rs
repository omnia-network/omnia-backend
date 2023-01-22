use candid::Principal;
use hex;
use ic_cdk::{call, print, trap};
use rand::Rng;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

use crate::STATE;

pub type Salt = [u8; 32];
pub const EMPTY_SALT: Salt = [0; 32];

fn create_byte_vector<T: RngCore>(rng: &mut T) -> Vec<u8> {
    let mut random_bytes = rng.gen::<[u8; 16]>();
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

pub async fn generate_uuid() -> String {
    let mut rng = make_rng().await;

    let hex_vector = hex_encode(create_byte_vector(&mut rng));
    let mut uuid = String::new();
    for (index, byte) in hex_vector.iter().enumerate() {
        uuid.push_str(byte);
        if [3, 5, 7, 9].contains(&index) {
            uuid.push('-');
        }
    }
    uuid
}

fn get_seed() -> Option<Salt> {
    STATE.with(|s| {
        let salt = s.borrow().random_seed;
        if salt == EMPTY_SALT {
            None
        } else {
            Some(salt)
        }
    })
}

// Get a random number generator based on 'raw_rand'
pub async fn make_rng() -> ChaCha20Rng {
    let mut seed = get_seed();

    if let None = seed {
        print("Random seed is not set");

        let raw_rand: Vec<u8> = match call(Principal::management_canister(), "raw_rand", ()).await {
            Ok((res,)) => res,
            Err((_, err)) => trap(&format!("failed to get seed: {}", err)),
        };
        let new_seed = raw_rand[..].try_into().unwrap_or_else(|_| {
            trap(&format!(
                    "when creating seed from raw_rand output, expected raw randomness to be of length 32, got {}",
                    raw_rand.len()
                    ));
        });

        // TODO: save the seed to avoid calling management_canister every time
        // STATE.with(|s| {
        //     s.borrow_mut().random_seed = new_seed;
        // });

        seed = Some(new_seed);
    }

    ChaCha20Rng::from_seed(seed.unwrap())
}
