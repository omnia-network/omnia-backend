use std::collections::BTreeMap;

use candid::candid_method;
use omnia_types::http::{
    HttpRequest,
    ParsedHttpRequestBody,
    HttpResponse,
    CONTENT_TYPE_HEADER_KEY,
    ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY,
    CONNECTION_HEADER_KEY,
    RequesterInfo,
};
use crate::utils::get_database_principal;

use ic_cdk::api::{call::call, time};
use ic_cdk_macros::{update, query};
use serde_json::from_slice;

#[query]
#[candid_method(query)]
fn http_request(req: HttpRequest) -> HttpResponse {

    // only allow POST method
    if req.method != "POST" {
        return HttpResponse {
            status_code: 405,
            headers: vec![
                (String::from(CONTENT_TYPE_HEADER_KEY), String::from("plain/text")),
                (String::from(ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY), String::from("*"))
            ],
            body: "Method Not Allowed".into(),
            streaming_strategy: None,
            upgrade: None,
        };
    }

    // this response is directed to the boundary node so that it can upgrade the initial query request "http_request" to an upgrade request "http_request_upgrade"
    HttpResponse {
        status_code: 101, // this is the HTTP status code to request an Upgrade of the protocol (it's anyway ignored by the Boundary node)
        headers: vec![
            // this header is optional and we use it just to explain which protocol we are upgrading to
            (String::from(CONNECTION_HEADER_KEY), String::from("IC_http_update_request")),
        ],
        body: "".into(),
        streaming_strategy: None,
        upgrade: Some(true),
    }
}

#[update]
#[candid_method(update)]
async fn http_request_update(req: HttpRequest) -> HttpResponse {

    let headers = req.headers.into_iter().fold(BTreeMap::new(), |mut headers, (header, value)| {
        headers.insert(header, value);
        headers
    });

    let parsed_body: ParsedHttpRequestBody = from_slice(&req.body.unwrap()).unwrap();

    let requester_info = RequesterInfo {
        ip: headers.get(&String::from("x-real-ip")).unwrap().to_owned(),
        timestamp: time(),
    };

    let _: (Option<RequesterInfo>, ) = call(
        get_database_principal(),
        "initNonceToIp",
        (
            parsed_body.nonce.to_string(),
            Box::new(requester_info),
        ),
    ).await.unwrap();

    // this is the response that the client actually get, even if the client called "http_requst"
    HttpResponse {
        status_code: 200,
        headers: vec![
            (String::from(CONTENT_TYPE_HEADER_KEY), String::from("plain/text")),
            (String::from(ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY), String::from("*"))
        ],
        body: "".into(),
        streaming_strategy: None,
        upgrade: None,
    }
}