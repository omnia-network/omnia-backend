import React, { useContext } from 'react';
import { EnvironmentQRCode } from './EnvironmentQRCode';
import { RegisterEnvironment } from './RegisterEnvironment';
import EnvironmentContext from '../contexts/EnvironmentContext';

const Home = () => {
  const { envData } = useContext(EnvironmentContext);

  return (
    <div style={{ textAlign: 'center' }}>
      {!envData ? (
        <RegisterEnvironment />
      ) : (
        <EnvironmentQRCode />
      )}
    </div>
  );
}

export default Home;
