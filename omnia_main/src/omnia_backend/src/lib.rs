mod stores;
mod manager;
mod user;
mod utils;

use utils::update_database_principal;

// to deploy this canister with the database principal id as init argument, use
// dfx deploy --argument '("<database-canister-id>")'
#[ic_cdk_macros::init]
fn init(arg: Option<String>) {
    ic_cdk::print(format!("Init canister..."));
    update_database_principal(arg);
}

#[ic_cdk_macros::post_upgrade]
fn post_upgrade(arg: Option<String>) {
    ic_cdk::print(format!("Post upgrade canister..."));
    update_database_principal(arg);
}
