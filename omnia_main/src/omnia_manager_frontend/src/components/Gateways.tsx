import { useContext, useEffect, useState } from "react";
import { omnia_backend } from "../../../declarations/omnia_backend";
import { Registeredgateway } from "../../../declarations/omnia_backend/omnia_backend.did";
import EnvironmentContext from "../contexts/EnvironmentContext";
import { handleError } from "../services/errors";
import { getGatewaysOfEnvironment } from "../services/gateways";
import { resultParser } from "../utils/resultParser";
import DataView from "./DataView";
import Devices from "./Devices";

interface IProps { };

const Gateways: React.FC<IProps> = () => {
  const [gatewayNameInput, setGatewayNameInput] = useState("");
  const [gatewayUidInput, setGatewayUidInput] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [isInitialLoading, setIsInitialLoading] = useState(false);
  const [gateways, setGateways] = useState<Registeredgateway[]>([]);
  const { envData } = useContext(EnvironmentContext);

  const registerGateway = async () => {

    if (!gatewayNameInput) {
      alert("Please enter a gateway name");
      return;
    }

    if (!gatewayUidInput) {
      alert("Please enter an initialized gateway UUID");
      return;
    }

    if (gateways.findIndex((g) => g.gateway_name === gatewayNameInput) !== -1) {
      alert("Gateway name already exists");
      return;
    }

    try {
      setIsLoading(true);

      const initGatewayUid = await omnia_backend.initGateway();

      const res = resultParser(await omnia_backend.registerGateway({
        env_uid: envData!.env_uid,
        gateway_name: gatewayNameInput,
        gateway_uid: initGatewayUid,
      }));

      if (res.error) {
        alert(res.error);
      } else {
        console.log("Gateway registered", res.data);
        // reload gateways from local storage
        setGateways(await getGatewaysOfEnvironment(envData!.env_uid));
      }

      // clear input
      setGatewayNameInput("");
      setGatewayUidInput("");
    } catch (e) {
      handleError(e);
    }

    setIsLoading(false);
  };

  useEffect(() => {
    setIsInitialLoading(true);

    getGatewaysOfEnvironment(envData!.env_uid)
      .then((res) => {
        setGateways(res);
        setIsInitialLoading(false);
      });
  }, [envData]);

  return (
    <div>
      <h2>Gateways</h2>

      <div>
        <h2>Register a new gateway</h2>
        <p><small>Generate a gateway uid from omnia_backend canister Candid UI</small></p>
        <input
          type="text"
          placeholder="Gateway name..."
          value={gatewayNameInput}
          onChange={(e) => setGatewayNameInput(e.target.value)}
        />
        <input
          type="text"
          placeholder="Gateway uid..."
          value={gatewayUidInput}
          onChange={(e) => setGatewayUidInput(e.target.value)}
        />
        <button
          onClick={registerGateway}
          disabled={isLoading}
        >
          {isLoading ? "Loading..." : "Register"}
        </button>
      </div>

      <h2>Gateway registered ({!isInitialLoading ? gateways.length : '...'})</h2>
      {!isInitialLoading && gateways.map((gateway) => (
        <div>
          <h3>{gateway.gateway_name}</h3>
          <DataView data={gateway} />
          <Devices gateway_uid={gateway.gateway_uid} />
        </div>
      ))}
    </div>
  );
};

export default Gateways;
