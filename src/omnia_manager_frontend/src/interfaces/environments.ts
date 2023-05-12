import { EnvironmentCreationResult } from "../../../declarations/omnia_backend/omnia_backend.did";

export type IEnvironmentData = EnvironmentCreationResult;

export interface IEnvironmentContext {
  envData: IEnvironmentData | null;
  setEnvData: (envData: IEnvironmentData) => void;
};
