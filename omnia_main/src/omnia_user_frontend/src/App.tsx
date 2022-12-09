import React, { FormEvent, useEffect, useRef, useState } from 'react';
import logo from './logo.svg';
import './App.css';
import { omnia_backend } from "../../declarations/omnia_backend";
import { getAuthClient } from './services/authClient';
import { QrReader } from 'react-qr-reader';

const App = () => {
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [environmentUid, setEnvironmentUid] = useState<string>('');
  const inputRef = useRef<HTMLInputElement>(null);
  const [isLogged, setIsLogged] = useState(false);
  const [scannedQrCode, setScannedQrCode] = useState<any>('');

  const onFormSumbit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    setIsSubmitting(true);

    const name = inputRef.current?.value;

    if (!name) {
      return;
    }

    try {
      // const uid = await omnia_backend.set_environment_uid(name);
      setEnvironmentUid(uid);
    } catch (e: any) {
      setEnvironmentUid(e.message);
    }

    setIsSubmitting(false);
  };

  const handleLogin = async () => {
    const authClient = await getAuthClient();
    authClient.login({
      async onSuccess() {
        // console.log("You are logged in as", await omnia_backend.whoami());
        setIsLogged(true);
      },
      onError(error) {
        console.log(error);
      },
      identityProvider: `http://localhost:4943?canisterId=${process.env.INTERNET_IDENTITY_CANISTER_ID}`
    });
  }

  useEffect(() => {
    getAuthClient().then(async (authClient) => {
      console.log("Auth Client initialized", authClient);
      const isAuth = await authClient.isAuthenticated();
      if (isAuth) {
        // console.log("You are logged in as", await omnia_backend.whoami());
      }
      setIsLogged(isAuth);
    });
  }, []);

  return (
    <div className="App">
      {isLogged
        ? (
          <div>
            <img src={logo} alt="DFINITY logo" />
            <br />
            <br />
            <form action="#" onSubmit={onFormSumbit}>
              <label htmlFor="name">Enter UID: &nbsp;</label>
              <input ref={inputRef} id="name" alt="Name" type="text" />
              <button type="submit" disabled={isSubmitting}>Enter</button>
            </form>
            <h2 style={{ textAlign: 'center' }}>{environmentUid}</h2>
            <h2 style={{ textAlign: 'center' }}>{scannedQrCode.toString()}</h2>
            <QrReader
              videoContainerStyle={{ paddingTop: 0 }}
              videoStyle={{ width: '50%', height: 'initial', margin: '20px auto', position: 'relative' }}
              onResult={(qrCode, error) => {

                if (error) {
                  console.info(error);
                  return;
                }

                if (qrCode) {
                  setScannedQrCode(qrCode);
                }
              }}
              constraints={{ facingMode: 'environment' }}  // use 'user' for front view
            />
          </div>
        )
        : <div>
          <button onClick={handleLogin}>Login</button>
        </div>}
    </div>
  );
}

export default App;
