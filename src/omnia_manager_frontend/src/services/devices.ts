import { omnia_backend } from "../../../declarations/omnia_backend";
import { DeviceInfo } from "../../../declarations/omnia_backend/omnia_backend.did";
import { resultParser } from "../utils/resultParser";
import { handleError } from "./errors";

export const getDevicesOfGateway = async (envUid: string, gatewayUid: string): Promise<DeviceInfo[]> => {
  try {
    const res = resultParser(await omnia_backend.getDevices(envUid));

    if (res.error) {
      throw res.error;
    }

    return res.data!.filter((d) => d.gateway_uid === gatewayUid);
  } catch (e) {
    handleError(e);
  }

  return [];
};
