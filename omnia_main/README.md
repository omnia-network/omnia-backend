# omnia_main

Here are some references on the SDKs and libraries we use in the codebase:

- [Internet Computer quick start](https://internetcomputer.org/docs/quickstart/quickstart-intro.html)
- [Internet Computer SDK Developer Tools](https://internetcomputer.org/docs/developers-guide/sdk-guide.html)
- [Rust Canister Devlopment Guide](https://internetcomputer.org/docs/rust-guide/rust-intro.html)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/candid-guide/candid-intro.html)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.ic0.app)

## Run canisters locally

### Generate types and did

Frontend canisters need Typescript types reflect backend canisters did interfaces and make the `@dfinity/agent-js` library work properly. To generate them simply run:
```bash
npm run generate:types
```

`dfx` utility still doesn't support automatic did interfaces generation. There's a workaround, which uses Rust tests and Rust DFINITY CDK under the hood: for each canister, there's a `save_candid` _test_ function that saves the candid interface in the canister's `.did` file. To run the did generation, run:
```bash
npm run generate:dids
```

### First deployment

First of all, start a local IC replica by running:
```bash
dfx start [--background] [--clean]
```
where `--background` is optional and makes it run in the background and `--clean` is optional and cleans the replica from previous deployments if there are.

There are two main canisters in the backend:
- **database**, which stores all data
- **omnia_backend**, which runs all the logics and exposes the methods to the frontend 

As you can guess, **omnia_backend** depends on **database** and needs to know the database canister id to make inter-canister calls work.

So, follow these steps to deploy everything correctly:

1. deploy **database** canister:
    ```bash
    dfx deploy database
    ```
    It'll give you an output similar to
    ```bash
    Deployed canisters.
    URLs:
      Backend canister via Candid interface:
        database: http://127.0.0.1:4943/?canisterId=ryjl3-tyaaa-aaaaa-aaaba-cai&id=rrkah-fqaaa-aaaaa-aaaaq-cai
    ```
    where the last `rrkah-fqaaa-aaaaa-aaaaq-cai` is the database canister id you need in the next steps
2. deploy all backend canisters with (replace `<database-canister-id>` with the actual database canister you obtained above):
    ```bash
    npm run generate-dids-and-deploy:backend -- <database-canister-id>
    ```
    this command first generates did interfaces accordingly and then builds and deploys all backend canisters

### Deployment

You have few commands available (you need to replace `<database-canister-id>` with the actual database canister id, see above):
- `npm run deploy -- <database-canister-id>`: deploys **all** canisters
- `npm run deploy:backend -- <database-canister-id>`: deploys **backend** canisters only
- `npm run generate-dids-and-deploy -- <database-canister-id>`: first generates dids and then deploys **all** canisters
- `npm run generate-dids-and-deploy:backend -- <database-canister-id>`: first generates dids and then deploys **backend** canisters only
