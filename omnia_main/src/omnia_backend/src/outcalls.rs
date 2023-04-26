use candid::Nat;
use ic_cdk::api::{
    management_canister::http_request::{HttpResponse, TransformArgs},
    print,
};
use ic_cdk_macros::query;

/// Strips all data that is not needed from the original response.
#[query]
fn transform(raw: TransformArgs) -> HttpResponse {
    let mut res = HttpResponse {
        status: raw.response.status.clone(),
        ..Default::default()
    };
    #[allow(clippy::cmp_owned)]
    if res.status >= Nat::from(200) && res.status < Nat::from(400) {
        // TODO: parse body properly
        res.body = vec![];
    } else {
        print(format!(
            "Received an error from HTTPS outcall: err = {:?}",
            raw
        ));
    }
    res
}
