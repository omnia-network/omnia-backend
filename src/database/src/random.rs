use core::time::Duration;
use getrandom::{register_custom_getrandom, Error};
use rand::{Rng, SeedableRng};

use crate::RNG_REF_CELL;

fn custom_getrandom(buf: &mut [u8]) -> Result<(), Error> {
    RNG_REF_CELL.with(|rng_ref_cell| {
        let mut rng = rng_ref_cell.borrow_mut();
        rng.fill(buf);
    });

    Ok(())
}

fn rng_seed() {
    ic_cdk::spawn(async move {
        let result: ic_cdk::api::call::CallResult<(Vec<u8>,)> =
            ic_cdk::api::call::call(candid::Principal::management_canister(), "raw_rand", ()).await;

        RNG_REF_CELL.with(|rng_ref_cell| {
            let mut rng = rng_ref_cell.borrow_mut();

            match result {
                Ok(randomness) => {
                    *rng = SeedableRng::from_seed(randomness.0[..].try_into().unwrap())
                }
                Err(err) => panic!("{:?}", err),
            };
        });
    });
}

register_custom_getrandom!(custom_getrandom);

/// Initialize the custom number generator by calling the management canister to get a random seed
pub fn init_rng() {
    ic_cdk_timers::set_timer(Duration::new(0, 0), rng_seed);
}
