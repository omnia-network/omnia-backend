/// The public IPv4 address of the proxy server, which forwards requests from Gateways to Backend.
/// For now it's hard coded, but in the future it could be fetched from the proxy server.
pub const OMNIA_PROXY_IPV4: &str = "3.70.56.192";

/// The host under which the proxy server is reachable. It should be used in Devices' URLs, otherwise the HTTPS certificate will not be valid.
pub const OMNIA_PROXY_HOST: &str = "proxy.omnia-iot.com";

/// The maximum number of requests that can be sent to Gateways with a single Access Key.
pub const ACCESS_KEY_REQUESTS_LIMIT: u32 = 10;
