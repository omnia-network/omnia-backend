import React, { useContext } from "react";
import QRCode from "react-qr-code";
import EnvironmentContext from "../contexts/EnvironmentContext";

interface IProps {}

export const EnvironmentQRCode: React.FC<IProps> = () => {
  const { envData } = useContext(EnvironmentContext);

  return (
    <div style={{ textAlign: 'center' }}>
      <h3>Scan the QR code to register yourself in environment:</h3>
      <h2>{envData!.envName}</h2>
      <QRCode value={JSON.stringify(envData)} />
    </div>
  );
};
