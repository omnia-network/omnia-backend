import React from 'react';
import Home from './components/Home';
import { EnvironmentContextProvider } from './contexts/EnvironmentContext';

function App() {

  return (
    <div style={{ textAlign: 'center' }}>
      <EnvironmentContextProvider>
        <Home />
      </EnvironmentContextProvider>
    </div>
  );
}

export default App;
