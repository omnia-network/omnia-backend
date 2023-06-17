use core::time::Duration;
use getrandom::{register_custom_getrandom, Error};
use ic_cdk::api::management_canister::main::raw_rand;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::cell::RefCell;

thread_local! {
    /* flexible */ pub static RNG_REF_CELL: RefCell<StdRng> = RefCell::new(SeedableRng::from_seed([0_u8; 32]));
}

fn custom_getrandom(buf: &mut [u8]) -> Result<(), Error> {
    RNG_REF_CELL.with(|rng_ref_cell| {
        let mut rng = rng_ref_cell.borrow_mut();
        rng.fill(buf);
    });

    Ok(())
}

fn rng_seed() {
    ic_cdk::spawn(async move {
        let rand_result = raw_rand().await;

        RNG_REF_CELL.with(|rng_ref_cell| {
            let mut rng = rng_ref_cell.borrow_mut();

            match rand_result {
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
