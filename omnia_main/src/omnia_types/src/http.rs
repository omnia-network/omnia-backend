use candid::CandidType;
use serde::{Deserialize, Serialize};

pub const CONTENT_TYPE_HEADER_KEY: &str = "content-type";

pub const ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY: &str = "Access-Control-Allow-Origin";

pub const CONNECTION_HEADER_KEY: &str = "Connection";

pub type HttpHeader = (String, String);
pub type CanisterCallNonce = String;
pub type RequesterIp = String;

#[derive(CandidType, Deserialize, Debug, PartialEq, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HttpHeader>,
    pub body: Option<Vec<u8>>,
    pub upgrade: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct ParsedHttpRequestBody {
    pub nonce: String,
}

#[derive(CandidType, Deserialize, Debug, PartialEq, Clone)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HttpHeader>,
    pub body: Vec<u8>,
    pub streaming_strategy: Option<String>,
    pub upgrade: Option<bool>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct RequesterInfo {
    pub requester_ip: RequesterIp,
    pub timestamp: u64,
}