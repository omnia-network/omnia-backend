export interface IEnvironmentData {
  envName: string;
};

export interface IEnvironmentContext {
  envData: IEnvironmentData | null;
  setEnvData: (envData: IEnvironmentData) => void;
};
