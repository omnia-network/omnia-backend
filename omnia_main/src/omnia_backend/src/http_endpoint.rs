use std::collections::BTreeMap;

use crate::{rdf::execute_sparql_query, utils::get_database_principal};
use candid::candid_method;
use omnia_types::http::{
    HttpRequest, HttpResponse, IpChallengeValue, ParsedHttpRequestBody,
    ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY, CONNECTION_HEADER_KEY, CONTENT_TYPE_HEADER_KEY,
};
use omnia_utils::net::is_proxy_ip;

use ic_cdk::api::{call::call, time};
use ic_cdk_macros::{query, update};
use serde_json::from_slice;

#[query]
#[candid_method(query)]
fn http_request(req: HttpRequest) -> HttpResponse {
    // only allow POST method
    if req.method != "POST" {
        return HttpResponse {
            status_code: 405,
            headers: vec![
                (
                    String::from(CONTENT_TYPE_HEADER_KEY),
                    String::from("plain/text"),
                ),
                (
                    String::from(ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY),
                    String::from("*"),
                ),
            ],
            body: "Method Not Allowed".into(),
            streaming_strategy: None,
            upgrade: None,
        };
    }

    if req.url.starts_with("/sparql/query") {
        let parsed_body = String::from_utf8(req.body.clone().unwrap()).unwrap();
        return match execute_sparql_query(parsed_body) {
            Ok(query_result) => HttpResponse {
                status_code: 200,
                headers: vec![
                    (
                        String::from(CONTENT_TYPE_HEADER_KEY),
                        String::from("application/json"),
                    ),
                    (
                        String::from(ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY),
                        String::from("*"),
                    ),
                ],
                body: query_result.into(),
                streaming_strategy: None,
                upgrade: None,
            },
            Err(e) => HttpResponse {
                status_code: 500,
                headers: vec![
                    (
                        String::from(CONTENT_TYPE_HEADER_KEY),
                        String::from("plain/text"),
                    ),
                    (
                        String::from(ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY),
                        String::from("*"),
                    ),
                ],
                body: format!("Error: {:?}", e).into(),
                streaming_strategy: None,
                upgrade: None,
            },
        };
    } else if req.url.starts_with("/ip-challenge") {
        // this response is directed to the boundary node so that it can upgrade the initial query request "http_request" to an upgrade request "http_request_upgrade"
        return HttpResponse {
            status_code: 101, // this is the HTTP status code to request an Upgrade of the protocol (it's anyway ignored by the Boundary node)
            headers: vec![
                // this header is optional and we use it just to explain which protocol we are upgrading to
                (
                    String::from(CONNECTION_HEADER_KEY),
                    String::from("IC_http_update_request"),
                ),
            ],
            body: "".into(),
            streaming_strategy: None,
            upgrade: Some(true),
        };
    }

    HttpResponse {
        status_code: 404,
        headers: vec![
            // this header is optional and we use it just to explain which protocol we are upgrading to
            (
                String::from(CONTENT_TYPE_HEADER_KEY),
                String::from("plain/text"),
            ),
            (
                String::from(ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY),
                String::from("*"),
            ),
        ],
        body: "Not Found".into(),
        streaming_strategy: None,
        upgrade: None,
    }
}

#[update]
#[candid_method(update)]
async fn http_request_update(req: HttpRequest) -> HttpResponse {
    // we check if the request has valid headers, otherwise we return an error
    // in order to have valid headers, the request must have the following headers:
    // - "x-forwarded-for" (mandatory): it contains the list of IP addresses of the proxies that the HTTP message went through.
    //   The list has this format: "client, proxy1, proxy2, ..., proxyN"
    //   Last IP of the list must either be:
    //      - IP address of the Omnia Proxy server, if the request is coming from a Gateway connected to Omnia Proxy server
    //      - IP address of the client (Gateway, User frontend, Manager frontend, etc.), if the request is coming directly from a client not connected to Omnia Proxy
    // - "x-proxied-for" (mandatory if the request is coming from Omnia Proxy): it contains the IP address of the client that sent the request to Omnia Proxy
    // - "x-peer-id" (mandatory if the request is coming from Omnia Proxy): it contains the ID that Omnia Proxy assigned to the Gateway that sent the request (this is needed to send an HTTP request to the proxied Gateway)
    let headers = req
        .headers
        .into_iter()
        .fold(BTreeMap::new(), |mut headers, (header, value)| {
            headers.insert(header, value);
            headers
        });

    // first check if the request has the x-forwarded-for header
    if !headers.contains_key("x-forwarded-for") {
        return HttpResponse {
            status_code: 400,
            headers: vec![
                (
                    String::from(CONTENT_TYPE_HEADER_KEY),
                    String::from("plain/text"),
                ),
                (
                    String::from(ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY),
                    String::from("*"),
                ),
            ],
            body: "Bad Request: missing x-forwarded-for header".into(),
            streaming_strategy: None,
            upgrade: None,
        };
    }

    // then, we have to split the x-forwarded-for header by comma to check if the last IP is the IP of the Omnia Proxy server or the IP of the client
    let x_forwarded_for: Vec<String> = headers
        .get("x-forwarded-for")
        .unwrap()
        .to_owned()
        .split(',')
        .map(|ip| ip.trim().to_owned())
        .collect();

    let (requester_ip, proxied_gateway_uid): (String, Option<String>) = match x_forwarded_for.last()
    {
        Some(ip) => {
            // if the last IP is the IP of the Omnia Proxy server, then we have to check if the request has the x-proxied-for and x-peer-id headers
            if is_proxy_ip(ip.to_owned()) {
                // check if the request has the x-proxied-for header
                if !headers.contains_key("x-proxied-for") {
                    return HttpResponse {
                        status_code: 400,
                        headers: vec![
                            (
                                String::from(CONTENT_TYPE_HEADER_KEY),
                                String::from("plain/text"),
                            ),
                            (
                                String::from(ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY),
                                String::from("*"),
                            ),
                        ],
                        body: "Bad Request: missing x-proxied-for header".into(),
                        streaming_strategy: None,
                        upgrade: None,
                    };
                }

                // check if the request has the x-peer-id header
                if !headers.contains_key("x-peer-id") {
                    return HttpResponse {
                        status_code: 400,
                        headers: vec![
                            (
                                String::from(CONTENT_TYPE_HEADER_KEY),
                                String::from("plain/text"),
                            ),
                            (
                                String::from(ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY),
                                String::from("*"),
                            ),
                        ],
                        body: "Bad Request: missing x-peer-id header".into(),
                        streaming_strategy: None,
                        upgrade: None,
                    };
                }

                // if the request has the x-proxied-for and x-peer-id headers, then we can return the IP address of the client that sent the request to Omnia Proxy
                (
                    headers.get("x-proxied-for").unwrap().to_owned(),
                    Some(headers.get("x-peer-id").unwrap().to_owned()),
                )
            } else {
                // if the last IP is not the IP of the Omnia Proxy server, then it must be the IP of the client and thus we can return it
                (ip.to_owned(), None)
            }
        }
        None => {
            return HttpResponse {
                status_code: 400,
                headers: vec![
                    (
                        String::from(CONTENT_TYPE_HEADER_KEY),
                        String::from("plain/text"),
                    ),
                    (
                        String::from(ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY),
                        String::from("*"),
                    ),
                ],
                body: "Bad Request: missing x-forwarded-for header".into(),
                streaming_strategy: None,
                upgrade: None,
            };
        }
    };

    let parsed_body: ParsedHttpRequestBody = from_slice(&req.body.unwrap()).unwrap();

    let requester_info = IpChallengeValue {
        requester_ip,
        is_proxied: proxied_gateway_uid.is_some(),
        proxied_gateway_uid,
        timestamp: time(),
    };

    ((),) = call(
        get_database_principal(),
        "initNonceToIp",
        (parsed_body.nonce.to_string(), Box::new(requester_info)),
    )
    .await
    .unwrap();

    // this is the response that the client actually get, even if the client called "http_requst"
    HttpResponse {
        status_code: 200,
        headers: vec![
            (
                String::from(CONTENT_TYPE_HEADER_KEY),
                String::from("plain/text"),
            ),
            (
                String::from(ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY),
                String::from("*"),
            ),
        ],
        body: "".into(),
        streaming_strategy: None,
        upgrade: None,
    }
}
