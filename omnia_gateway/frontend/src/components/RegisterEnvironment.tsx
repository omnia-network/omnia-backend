import React, { useContext, useState } from "react";
import EnvironmentContext from "../contexts/EnvironmentContext";

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
