use std::cmp::Ordering;

use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::errors::GenericError;

pub const CONTENT_TYPE_HEADER_KEY: &str = "content-type";

pub const ACCESS_CONTROL_ALLOW_ORIGIN_HEADER_KEY: &str = "Access-Control-Allow-Origin";

pub const CONNECTION_HEADER_KEY: &str = "Connection";

pub type HttpHeader = (String, String);
pub type IpChallengeNonce = String;
pub type Ip = String;
pub type ProxiedGatewayUID = String;

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
    pub requester_ip: Ip,
    pub proxy_ip: Option<Ip>,
    pub proxied_gateway_uid: Option<ProxiedGatewayUID>,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct IpChallengeIndex {
    pub nonce: IpChallengeNonce,
}

impl Ord for IpChallengeIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.nonce.cmp(&other.nonce)
    }
}

pub type IpChallengeValueResult = Result<IpChallengeValue, GenericError>;