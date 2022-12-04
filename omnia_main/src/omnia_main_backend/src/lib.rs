use candid::Principal;
use ic_cdk::api;

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk_macros::query]
fn whoami() -> Principal {
    api::caller()
}