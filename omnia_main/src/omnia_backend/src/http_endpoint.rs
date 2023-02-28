use candid::candid_method;
use omnia_types::http::{
    HttpRequest,
    HttpResponse,
    CONTENT_TYPE_HEADER_KEY,
    ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY,
};

use ic_cdk::print;
use ic_cdk_macros::query;

#[query]
#[candid_method(query)]
fn http_request(req: HttpRequest) -> HttpResponse {
    // only allow GET method
    if req.method != "GET" {
        return HttpResponse {
            status_code: 405,
            headers: vec![
                (String::from(CONTENT_TYPE_HEADER_KEY), String::from("plain/text")),
                (String::from(ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY), String::from("*"))
            ],
            body: "Method Not Allowed".into(),
            streaming_strategy: None,
        };
    }

    print(format!("Request headers: {:?}", req.headers));


    HttpResponse {
        status_code: 200,
        headers: vec![
            (String::from(CONTENT_TYPE_HEADER_KEY), String::from("plain/text")),
            (String::from(ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY), String::from("*"))
        ],
        body: "A stecca dio cane".into(),
        streaming_strategy: None,
    }
}