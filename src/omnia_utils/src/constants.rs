/// The public IPv4 address of the proxy server, which forwards requests from Gateways to Backend.
/// For now it's hard coded, but in the future it could be fetched from the proxy server.
pub const OMNIA_PROXY_IPV4: &str = "3.70.56.192";

/// The host under which the proxy server is reachable. It should be used in Devices' URLs, otherwise the HTTPS certificate will not be valid.
pub const OMNIA_PROXY_HOST: &str = "proxy.omnia-iot.com";
