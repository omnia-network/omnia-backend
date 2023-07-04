use std::{borrow::Cow, cmp::Ordering};

use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::{Deserialize, Serialize};

use crate::{errors::GenericResult, MAX_STABLE_BTREE_MAP_SIZE};

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
    /// If this is a proxied request, this is the UID of the proxied gateway.
    pub proxied_gateway_uid: Option<ProxiedGatewayUID>,
    /// This is used around the codebase to determine if a request is proxied or not.
    /// Not sure if it's a necessary field, but makes it easier to read.
    pub is_proxied: bool,
    pub timestamp: u64,
}

impl Storable for IpChallengeValue {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for IpChallengeValue {
    const MAX_SIZE: u32 = MAX_STABLE_BTREE_MAP_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize, PartialEq, Eq)]
pub struct IpChallengeIndex {
    pub nonce: IpChallengeNonce,
}

impl Ord for IpChallengeIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.nonce.cmp(&other.nonce)
    }
}

impl PartialOrd for IpChallengeIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Storable for IpChallengeIndex {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for IpChallengeIndex {
    const MAX_SIZE: u32 = MAX_STABLE_BTREE_MAP_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

pub type IpChallengeValueResult = GenericResult<IpChallengeValue>;
