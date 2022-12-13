import React, { FormEvent, useContext, useRef, useState } from 'react';
import { QrReader } from 'react-qr-reader';

import './Home.css';
import ProfileContext from '../contexts/ProfileContext';
import { omnia_backend } from '../../../declarations/omnia_backend';
import { EnvironmentInfo } from '../../../declarations/omnia_backend/omnia_backend.did';
import GenericMessage from './GenericMessage';

const Home = () => {
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [environmentInfo, setEnvironmentInfo] = useState<EnvironmentInfo | null>(null);
  const [isScanning, setIsScanning] = useState(true);
  const inputRef = useRef<HTMLInputElement>(null);
  const [genericMessage, setGenericMessage] = useState('');
  const { profile, isLoading, login, fetchProfileFromCanister } = useContext(ProfileContext);

  const onFormSumbit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    setIsSubmitting(true);

    console.log(e);

    const envId = inputRef.current?.value;

    if (!envId) {
      return;
    }

    setIsScanning(false);

    await setEnvironment(parseInt(envId, 10));

    setIsSubmitting(false);
  };

  const handleScan = async (data: any) => {
    // data is a JSON string with the following structure:
    // {
    //   "env_uid": "env_uid",
    //   "envName": "env_name",
    // }
    if (data && !environmentInfo && isScanning) {
      setIsScanning(false);
      await setEnvironment(JSON.parse(data).env_uid);
    }
  };

  const setEnvironment = async (envId: number) => {

    if (environmentInfo) {
      return;
    }

    try {
      setGenericMessage('Got environment ID, setting environment on canister...');
      const envInfo = await omnia_backend.setEnvironment(envId);
      // fetch profile again to get the new environment info
      await fetchProfileFromCanister();

      console.log('setEnvironment: envInfo', envInfo);

      setEnvironmentInfo(envInfo);

      setGenericMessage('');
    } catch (e: any) {
      setGenericMessage(e.message);

      setIsScanning(true);
    }

  };

  const resetEnvironment = async () => {
    try {
      setGenericMessage('Resetting environment on canister...');
      const envInfo = await omnia_backend.resetEnvironment();
      // fetch profile again to get the new environment info
      await fetchProfileFromCanister();

      console.log('resetEnvironment: envInfo', envInfo);

      setEnvironmentInfo(null);

      setGenericMessage('');
    } catch (e: any) {
      setGenericMessage(e.message);
    }

    setIsScanning(true);
  };

  const handleLogin = async () => {
    await login();
    setGenericMessage('Login successful!');

    setTimeout(() => {
      setGenericMessage('');
    }, 2000);
  };

  if (isLoading) {
    return (
      <div className="App">
        <h2>Loading auth and profile...</h2>
      </div>
    );
  }

  return (
    <div className="App">
      <GenericMessage message={genericMessage} />
      {!!profile
        ? (
          <div>
            <p>Principal ID: <b>{profile.user_principal_id}</b></p>
            <p>User in environment ID: <b>{profile.environment_uid[0] || 'null'}</b></p>
            {(!!environmentInfo || profile.environment_uid[0])
              ? (
                <div>
                  <h2>Environment Info</h2>
                  <p>Environment UID: <b>{environmentInfo?.env_uid || profile.environment_uid[0]}</b></p>
                  <p>Environment Name: <b>{environmentInfo?.env_name || 'getEnvironmentInfo canister method is missing here'}</b></p>
                  <button onClick={resetEnvironment}>Reset Environment</button>
                  <p>Agents available:</p>
                </div>
              )
              : (
              <div>
                <form action="#" onSubmit={onFormSumbit}>
                  <label htmlFor="envId">Enter UID: &nbsp;</label>
                  <input ref={inputRef} id="envId" alt="Environment ID" type="number" placeholder="Environment ID..." />
                  <button type="submit" disabled={isSubmitting}>Enter</button>
                </form>
                <QrReader
                  videoContainerStyle={{ paddingTop: 0 }}
                  scanDelay={1000}
                  videoStyle={{ width: '50%', height: 'initial', margin: '20px auto', position: 'relative' }}
                  onResult={(qrCode, error) => {
    
                    if (error) {
                      console.info("QR code scan error:", error);
                      return;
                    }
    
                    handleScan(qrCode);
                  }}
                  constraints={{ facingMode: 'environment' }}  // use 'user' for front view
                />
              </div>
              )
            }
          </div>
        )
        : <div>
          <button onClick={handleLogin}>Login</button>
        </div>
      }
    </div>
  );
}

export default Home;
