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

/// TODO: ideally the device URL is created by reading the Gateway WoT servient index page, so we shouldn't need this function
pub fn get_device_url(gateway_url: String, device_uid: String) -> String {
    format!("{}/{}", gateway_url, device_uid)
}
