import { DeviceAffordances } from "../../src/declarations/omnia_backend/omnia_backend.did";

// The public IPv4 address of the Omnia Proxy, which forwards requests from and to Gateways.
export const OMNIA_PROXY_IPV4 = "3.70.56.192";
// The host under which the Omnia Proxy is reachable.
export const OMNIA_PROXY_HOST = "proxy.omnia-iot.com";

// test timeouts
export const LONG_TEST_TIMEOUT = 120_000;

// test data
export const ENVIRONMENT_NAME = "test_environment";
export const GATEWAY1_NAME = "test_gateway1";
export const GATEWAY2_NAME = "test_gateway2";
export const TOTAL_GATEWAYS_IN_ENV = 2;
export const DEVICE1_NAME = "test_device1";
export const DEVICE2_NAME = "test_device2";
export const TOTAL_DEVICES_IN_ENV = 2;
export const DEVICE_PAIRING_PAYLOAD = "test_device_pairing_payload";
// mapped from the Thing Description of the device and must be SAREF verbs
// backend accepts both saref:... and the verb itself
export const DEVICE_AFFORDANCES: DeviceAffordances = {
  properties: [
    "saref:OnOffState",
  ],
  actions: [
    "OnCommand",
  ],
};
export const DEVICE_AFFORDANCE_VALUE_TUPLE: [string, string] = [
  "td:hasPropertyAffordance",
  "saref:OnOffState"
];

export const ACCESS_KEY_PRICE = BigInt(1_000_000);
