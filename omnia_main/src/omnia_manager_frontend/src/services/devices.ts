import { omnia_backend } from "../../../declarations/omnia_backend";
import { DeviceInfo } from "../../../declarations/omnia_backend/omnia_backend.did";
import { handleError } from "./errors";

export const getDevicesOfGateway = async (envUid: string, gatewayUid: string): Promise<DeviceInfo[]> => {
  try {
    const res = await omnia_backend.getDevices(envUid);

    return res.filter((d) => d.gateway_uid === gatewayUid);
  } catch (e) {
    handleError(e);
  }

  return [];
};
