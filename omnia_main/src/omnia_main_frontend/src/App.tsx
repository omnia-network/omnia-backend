import React, { FormEvent, useRef, useState } from 'react';
import logo from './logo.svg';
import './App.css';
import { omnia_main_backend } from "../../declarations/omnia_main_backend";

const App = () => {
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [result, setResult] = useState<string>('');
  const inputRef = useRef<HTMLInputElement>(null);

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

  return (
    <div className="App">
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
  );
}

export default App;
