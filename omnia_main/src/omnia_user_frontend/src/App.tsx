import React from 'react';
import Home from './components/Home';
import { ProfileProvider } from './contexts/ProfileContext';

import './App.css';

const App = () => {
  return (
    <div className="App">
      <ProfileProvider>
        <Home />
      </ProfileProvider>
    </div>
  );
}

export default App;
