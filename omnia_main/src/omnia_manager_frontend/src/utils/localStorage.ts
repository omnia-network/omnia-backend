import { IEnvironmentData } from "../interfaces/environments";

const ENV_DATA_KEY = "env_data";

export const saveEnvData = (envData: IEnvironmentData) => {
  localStorage.setItem(ENV_DATA_KEY, JSON.stringify(envData));
};

export const retrieveEnvData = (): IEnvironmentData | null => {

  const envDataStr = localStorage.getItem(ENV_DATA_KEY);

  if (envDataStr) {
    console.log("envData retrieved from localStorage", envDataStr);
    return JSON.parse(envDataStr);
  }

  return null;
};
