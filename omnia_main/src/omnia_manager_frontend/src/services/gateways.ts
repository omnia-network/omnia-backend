import { omnia_backend } from "../../../declarations/omnia_backend";
import { RegisteredGateway } from "../../../declarations/omnia_backend/omnia_backend.did";
import { resultParser } from "../utils/resultParser";
import { handleError } from "./errors";

export const getGatewaysOfEnvironment = async (envUid: string): Promise<RegisteredGateway[]> => {
  try {
    const res = resultParser(await omnia_backend.getGateways(envUid));

    if (res.error) {
      throw res.error;
    }

    return res.data!;
  } catch (e) {
    handleError(e);
  }

  return [];
};
