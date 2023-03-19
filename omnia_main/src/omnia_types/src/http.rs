use std::cmp::Ordering;

use candid::CandidType;
use serde::{Deserialize, Serialize};

pub const CONTENT_TYPE_HEADER_KEY: &str = "content-type";

pub const ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY: &str = "Access-Control-Allow-Origin";

pub const CONNECTION_HEADER_KEY: &str = "Connection";

pub type HttpHeader = (String, String);
pub type IpChallengeNonce = String;
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

#[derive(Clone, Default, CandidType, Serialize, Deserialize, Debug)]
pub struct IpChallengeValue {
    pub requester_ip: RequesterIp,
    pub timestamp: u64,
}

#[derive(Clone, Default, CandidType, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct IpChallengeIndex {
    pub nonce: IpChallengeNonce,
}

impl Ord for IpChallengeIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.nonce.cmp(&other.nonce)
    }
}
