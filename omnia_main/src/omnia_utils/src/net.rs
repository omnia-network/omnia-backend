use crate::constants::OMNIA_PROXY_IPV4;

/// TODO: move this in a better place
pub fn is_proxy_ip(ip: String) -> bool {
    ip == OMNIA_PROXY_IPV4
}

/// TODO: move this in a better place and definitely rename it
pub fn get_gateway_url(ip: String, is_proxied: bool) -> String {
    let address = match is_proxied {
        true => OMNIA_PROXY_IPV4,
        false => &ip,
    };
    format!("https://{address}")
}
