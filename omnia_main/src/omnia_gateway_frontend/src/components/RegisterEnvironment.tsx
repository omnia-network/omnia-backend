import React, { useContext, useState } from "react";
import { omnia_backend } from "../../../declarations/omnia_backend";
import EnvironmentContext from "../contexts/EnvironmentContext";
import { IEnvironmentData } from "../interfaces/environments";

interface IProps {}

export const RegisterEnvironment: React.FC<IProps> = () => {
  const [envName, setEnvName] = useState("");
  const { setEnvData } = useContext(EnvironmentContext);
  const [isLoading, setIsLoading] = useState(false);

  const handleEnvNameChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setEnvName(event.target.value);
  };

  const handleRegister = async () => {
    if (envName === "") {
      return;
    }

    setIsLoading(true);

    // register environment on canister
    // const res = await omnia_backend.registerEnvironment({
    //   env_name: envName,
    // });

    // const envDataRes: IEnvironmentData = {
    //   ...res,
    //   envName,
    // };

    // set env data in context
    // setEnvData(envDataRes);

    setIsLoading(false);
  };

  return (
    <div style={{ textAlign: 'center' }}>
      <h2>Register a new environment</h2>
      <input
        type="text"
        value={envName}
        onChange={handleEnvNameChange}
        placeholder="Environment name..."
      />
      <button
        onClick={handleRegister}
        disabled={isLoading}
      >
        {isLoading ? "Loading..." : "Register"}
      </button>
    </div>
  );
};
