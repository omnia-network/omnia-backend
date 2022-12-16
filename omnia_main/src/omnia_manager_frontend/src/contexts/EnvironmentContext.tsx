import React, { createContext, useEffect, useState } from "react";
import { IEnvironmentContext } from "../interfaces/environments";
import { retrieveEnvData, saveEnvData } from "../utils/localStorage";

const EnvironmentContext = createContext<IEnvironmentContext>({
  envData: null,
  setEnvData: () => {},
});

type IProps = {
  children?: React.ReactNode
};

export const EnvironmentContextProvider: React.FC<IProps> = ({children}) => {
  const [envData, _setEnvData] = useState<IEnvironmentContext["envData"] | null>(null);

  const setEnvData = (envData: IEnvironmentContext["envData"]) => {
    if (envData) {
      saveEnvData(envData);
    }
    _setEnvData(envData);
  };

  useEffect(() => {
    const envData = retrieveEnvData();
    if (envData) {
      _setEnvData(envData);
    }
  }, []);

  return (
    <EnvironmentContext.Provider value={{ envData, setEnvData }}>
      {children}
    </EnvironmentContext.Provider>
  );
};

export default EnvironmentContext;