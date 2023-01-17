import { DeviceRegistrationResult, GatewayRegistrationResult } from "../../../declarations/omnia_backend/omnia_backend.did";

const GATEWAYS_KEY = "gateways";
const DEVICES_KEY = "devices";

export const saveGateway = (gateway: GatewayRegistrationResult) => {

  const gateways = getGateways();

  if (gateways) {
    // Check if gateway already exists
    // If it does, update it
    const index = gateways.findIndex(g => g.gateway_uid === gateway.gateway_uid);
    if (index !== -1) {
      gateways[index] = gateway;
    } else {
      gateways.push(gateway);
    }
  } else {
    localStorage.setItem(GATEWAYS_KEY, JSON.stringify([gateway]));
    return;
  }

  localStorage.setItem(GATEWAYS_KEY, JSON.stringify(gateways));
}

export const getGateways = (): GatewayRegistrationResult[] => {
  const gateways = localStorage.getItem(GATEWAYS_KEY);
  return gateways ? JSON.parse(gateways) : [];
}

export const saveDevice = (device: DeviceRegistrationResult) => {
  const devices = getDevices();

  if (devices) {
    // Check if device already exists
    // If it does, update it
    const index = devices.findIndex(d => d.device_uid === device.device_uid);
    if (index !== -1) {
      devices[index] = device;
    } else {
      devices.push(device);
    }
  } else {
    localStorage.setItem(DEVICES_KEY, JSON.stringify([device]));
    return;
  }

  localStorage.setItem(DEVICES_KEY, JSON.stringify(devices));
}

export const getDevices = (): DeviceRegistrationResult[] => {
  const devices = localStorage.getItem(DEVICES_KEY);
  return devices ? JSON.parse(devices) : [];
}