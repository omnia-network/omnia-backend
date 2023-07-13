use std::env;

use candid::Principal;
use omnia_utils::ic::principal_to_account;

fn main() {
    let args: Vec<String> = env::args().collect();
    let principal_id = args.get(1).unwrap();
    println!("Input principal id: {principal_id}");

    println!(
        "{}",
        hex::encode(principal_to_account(
            Principal::from_text(principal_id).unwrap()
        ))
    );
}
