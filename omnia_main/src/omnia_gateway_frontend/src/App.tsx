import React, { useEffect, useState } from 'react';
import QRCode from "react-qr-code";
import { omnia_backend } from "../../declarations/omnia_backend";

function App() {

  const [deviceUid, setDeviceUid] = useState("");

  useEffect(() => {
    initDeviceUid();
  }, []);

  const initDeviceUid = async () => {
    setDeviceUid(await omnia_backend.get_device_uid());
  }

  return (
    <div>
      <h2>Scan the QR code to access this device</h2>
      <QRCode value={deviceUid} />
    </div>
  );
}

export default App;
