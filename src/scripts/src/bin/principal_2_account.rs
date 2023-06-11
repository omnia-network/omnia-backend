use std::env;

use candid::Principal;
use ic_ledger_types::{AccountIdentifier, Subaccount};

fn main() {
    let args: Vec<String> = env::args().collect();
    let principal_id = args.get(1).unwrap();
    println!("{principal_id}");

    println!(
        "{}",
        hex::encode(AccountIdentifier::new(
            &Principal::from_text(principal_id).expect("valid principal id"),
            &Subaccount([0; 32]),
        ))
    );
}
