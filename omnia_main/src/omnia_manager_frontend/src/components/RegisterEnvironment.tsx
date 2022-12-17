import React, { useContext, useState } from "react";
import { omnia_backend } from "../../../declarations/omnia_backend";
import EnvironmentContext from "../contexts/EnvironmentContext";
import ProfileContext from "../contexts/ProfileContext";

interface IProps {}

const RegisterEnvironment: React.FC<IProps> = () => {
  const [envName, setEnvName] = useState("");
  const { setEnvData } = useContext(EnvironmentContext);
  const { fetchProfileFromCanister } = useContext(ProfileContext);
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
    const envDataRes = await omnia_backend.createEnvironment({
      env_name: envName,
    });

    // fetch profile again to get the new environment info
    await fetchProfileFromCanister();

    // set env data in context
    setEnvData(envDataRes);

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

export default RegisterEnvironment;
