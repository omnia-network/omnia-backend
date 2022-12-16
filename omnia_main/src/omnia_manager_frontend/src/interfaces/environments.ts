import { EnvironmentCreationResult } from "../../../declarations/omnia_backend/omnia_backend.did";

export interface IEnvironmentData extends EnvironmentCreationResult {
  envName: string;
};

export interface IEnvironmentContext {
  envData: IEnvironmentData | null;
  setEnvData: (envData: IEnvironmentData) => void;
};
