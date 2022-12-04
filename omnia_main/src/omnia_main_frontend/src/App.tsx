import React, { FormEvent, useEffect, useRef, useState } from 'react';
import logo from './logo.svg';
import './App.css';
import { omnia_main_backend } from "../../declarations/omnia_main_backend";
import { getAuthClient } from './services/authClient';

const App = () => {
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [result, setResult] = useState<string>('');
  const inputRef = useRef<HTMLInputElement>(null);
  const [isLogged, setIsLogged] = useState(false);

  const onFormSumbit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    setIsSubmitting(true);

    const name = inputRef.current?.value;

    if (!name) {
      return;
    }

    try {
      // Interact with foo actor, calling the greet method
      const result = await omnia_main_backend.greet(name);
      setResult(result);
    } catch (e: any) {
      setResult(e.message);
    }
    
    setIsSubmitting(false);
  };

  const handleLogin = async () => {
    const authClient = await getAuthClient();
    authClient.login({
      async onSuccess() {
        console.log("You are logged in as", await omnia_main_backend.whoami());
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
        console.log("You are logged in as", await omnia_main_backend.whoami());
      }
      setIsLogged(isAuth);
    });
  }, []);

  return (
    <div className="App">
      {isLogged 
        ? <div>
          <img src={logo} alt="DFINITY logo" />
          <br />
          <br />
          <form action="#" onSubmit={onFormSumbit}>
            <label htmlFor="name">Enter your name: &nbsp;</label>
            <input ref={inputRef} id="name" alt="Name" type="text" />
            <button type="submit" disabled={isSubmitting}>Click Me!</button>
          </form>
          <section id="greeting">{result}</section>
        </div> 
        : <div>
          <button onClick={handleLogin}>Login</button>
        </div>}
    </div>
  );
}

export default App;
