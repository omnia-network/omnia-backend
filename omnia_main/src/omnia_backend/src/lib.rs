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

#[ic_cdk_macros::query]
fn get_device_uid() -> String {
    String::from("omnia_device_0")
}