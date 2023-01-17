import { useContext, useState } from "react";
import { omnia_backend } from "../../../declarations/omnia_backend";
import { GatewayRegistrationResult } from "../../../declarations/omnia_backend/omnia_backend.did";
import EnvironmentContext from "../contexts/EnvironmentContext";
import { getGateways, saveGateway } from "../services/localStorage";
import DataView from "./DataView";
import Devices from "./Devices";

interface IProps { };

const Gateways: React.FC<IProps> = () => {
  const [gatewayNameInput, setGatewayNameInput] = useState("");
  const [gatewayUidInput, setGatewayUidInput] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [gateways, setGateways] = useState<GatewayRegistrationResult[]>(getGateways());
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

      const res = await omnia_backend.registerGateway({
        env_uid: envData!.env_uid,
        gateway_name: gatewayNameInput,
        gateway_uid: gatewayUidInput,
      });

      if (res.length > 0) {
        console.log("Gateway registered", res[0]);
        // save gateway in local storage
        saveGateway(res[0]!);
      }

      // reload gateways from local storage
      setGateways(getGateways());
      // clear input
      setGatewayNameInput("");
      setGatewayUidInput("");
    } catch (e) {
      // TODO: handle error
      console.log("Error registering gateway", e);
    }

    setIsLoading(false);
  };

  return (
    <div>
      <h2>Gateways</h2>

      <div>
        <h2>Register a new gateway</h2>
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

      <h2>Gateway registered</h2>
      <p><code>omnia_backend.getGateways</code> missing here, loaded from localStorage</p>
      {gateways.map((gateway) => (
        <div>
          <h3>{gateway.gateway_name}</h3>
          <DataView data={gateway} />
          <Devices gateway_uid={gateway.gateway_uid}/>
        </div>
      ))}
    </div>
  );
};

export default Gateways;