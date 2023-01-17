import React, { useContext } from 'react';
import EnvironmentDashboard from './EnvironmentDashboard';
import RegisterEnvironment from './RegisterEnvironment';
import EnvironmentContext from '../contexts/EnvironmentContext';

const Home = () => {
  const { envData } = useContext(EnvironmentContext);

  return (
    <div style={{ textAlign: 'center' }}>
      {!envData ? (
        <RegisterEnvironment />
      ) : (
        <EnvironmentDashboard />
      )}
    </div>
  );
}

export default Home;
