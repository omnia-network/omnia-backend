import { omnia_backend } from "../../../declarations/omnia_backend";
import { GatewayInfo } from "../../../declarations/omnia_backend/omnia_backend.did";
import { handleError } from "./errors";

export const getGatewaysOfEnvironment = async (envUid: string): Promise<GatewayInfo[]> => {
  try {
    const res = await omnia_backend.getGateways(envUid);

    return res;
  } catch (e) {
    handleError(e);
  }

  return [];
};
