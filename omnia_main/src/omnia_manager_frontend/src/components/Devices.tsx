import { useContext, useState } from "react";
import { omnia_backend } from "../../../declarations/omnia_backend";
import { DeviceRegistrationResult, GatewayRegistrationResult } from "../../../declarations/omnia_backend/omnia_backend.did";
import EnvironmentContext from "../contexts/EnvironmentContext";
import { getDevices, saveDevice } from "../services/localStorage";
import DataView from "./DataView";

interface IProps {
  gateway_uid: GatewayRegistrationResult["gateway_uid"];
};

const Devices: React.FC<IProps> = ({ gateway_uid }) => {
  const [deviceNameInput, setDeviceNameInput] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [devices, setDevices] = useState<DeviceRegistrationResult[]>(getDevices());
  const { envData } = useContext(EnvironmentContext);

  const registerDevice = async () => {

    if (!deviceNameInput) {
      alert("Please enter a gateway name");
      return;
    }

    if (devices.findIndex((d) => d.device_name === deviceNameInput) !== -1) {
      alert("Device name already exists");
      return;
    }

    try {
      setIsLoading(true);

      const res = await omnia_backend.registerDevice({
        env_uid: envData!.env_uid,
        gateway_uid: gateway_uid,
        device_name: deviceNameInput,
      });

      console.log("Device registered", res);

      // save device in local storage
      saveDevice(res);
      // reload devices from local storage
      setDevices(getDevices());
      // clear input
      setDeviceNameInput("");
    } catch (e) {
      // TODO: handle error
      console.log("Error registering device", e);
    }

    setIsLoading(false);
  };

  return (
    <div style={{ marginLeft: 50 }}>
      <h2>Devices in this gateway</h2>

      <div>
        <p><code>omnia_backend.getDevices</code> missing here, loaded from localStorage</p>
        {devices.map((device) => <DataView data={device} />)}
        <h2>Register a new device</h2>
        <input
          type="text"
          placeholder="Device name..."
          value={deviceNameInput}
          onChange={(e) => setDeviceNameInput(e.target.value)}
        />
        <button
          onClick={registerDevice}
          disabled={isLoading}
        >
          {isLoading ? "Loading..." : "Register"}
        </button>
      </div>
    </div>
  );
};

export default Devices;