import { useContext, useEffect, useState } from "react";
import { omnia_backend } from "../../../declarations/omnia_backend";
import { DeviceInfo, GatewayInfo } from "../../../declarations/omnia_backend/omnia_backend.did";
import EnvironmentContext from "../contexts/EnvironmentContext";
import { getDevicesOfGateway } from "../services/devices";
import { handleError } from "../services/errors";
import DataView from "./DataView";

interface IProps {
  gateway_uid: GatewayInfo["gateway_uid"];
};

const Devices: React.FC<IProps> = ({ gateway_uid }) => {
  const [deviceNameInput, setDeviceNameInput] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [isInitialLoading, setIsInitialLoading] = useState(false);
  const [devices, setDevices] = useState<DeviceInfo[]>([]);
  const { envData } = useContext(EnvironmentContext);

  const registerDevice = async () => {

    if (!deviceNameInput) {
      alert("Please enter a device name");
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

      // reload devices from local storage
      setDevices(await getDevicesOfGateway(envData!.env_uid, gateway_uid));
      // clear input
      setDeviceNameInput("");
    } catch (e) {
      handleError(e);
    }

    setIsLoading(false);
  };

  useEffect(() => {

    setIsInitialLoading(true);

    getDevicesOfGateway(envData!.env_uid, gateway_uid)
      .then((res) => setDevices(res))
      .catch((e) => handleError(e))
      .then(() => setIsInitialLoading(false));
  }, [envData, gateway_uid]);

  return (
    <div style={{ marginLeft: 50 }}>
      <h2>Devices in this gateway ({!isInitialLoading ? devices.length : '...'})</h2>

      <div>
        {!isInitialLoading && devices.map((device) => <DataView data={device} />)}
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
