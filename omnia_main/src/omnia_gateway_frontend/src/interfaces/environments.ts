import { EnvironmentRegistrationResult } from "../../../declarations/omnia_backend/omnia_backend.did";

export interface IEnvironmentData extends EnvironmentRegistrationResult {
  envName: string;
};

export interface IEnvironmentContext {
  envData: IEnvironmentData | null;
  setEnvData: (envData: IEnvironmentData) => void;
};
