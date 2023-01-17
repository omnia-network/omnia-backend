import React from 'react';
import Home from './components/Home';
import { EnvironmentContextProvider } from './contexts/EnvironmentContext';
import { ProfileProvider } from './contexts/ProfileContext';

function App() {

  return (
    <div style={{ textAlign: 'center' }}>
      <ProfileProvider>
        <EnvironmentContextProvider>
          <Home />
        </EnvironmentContextProvider>
      </ProfileProvider>
    </div>
  );
}

export default App;
