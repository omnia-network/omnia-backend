import React, { useContext } from "react";
import QRCode from "react-qr-code";
import EnvironmentContext from "../contexts/EnvironmentContext";
import DataView from "./DataView";
import Gateways from "./Gateways";

import "./EnvironmentDashboard.css";

interface IProps { };

const EnvironmentDashboard: React.FC<IProps> = () => {
  const { envData } = useContext(EnvironmentContext);

  if (!envData) {
    return <div>Environment data not found</div>;
  }

  return (
    <div className="environment-dashboard-container">
      <h1>Environment Dashboard</h1>
      <div className="environment-dashboard-box-container">
        <div className="environment-dashboard-box" style={{width: "40%"}}>
          <h2>Environment Data</h2>
          <DataView data={envData} />

          <h3>Users need this QR code to register in your environment:</h3>
          <QRCode value={JSON.stringify(envData)} />
        </div>
        <div className="environment-dashboard-box">
          <Gateways />
        </div>
      </div>
    </div>
  );
};

export default EnvironmentDashboard;
